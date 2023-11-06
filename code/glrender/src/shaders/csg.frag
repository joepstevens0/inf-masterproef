#version 330 core
precision mediump float;
// precision highp usampler2D;

#define lightpos (vec3(5,3,-4)*2.)

#define SURFACE_DIST 0.001
#define MAX_STEPS 50u
#define MAX_RAYS 1u
#define MAX_DIST 100.0
#define MIN_DIST 0.0
#define MISS_sd sdCap(MAX_DIST+1., 0u, vec4(0))

// TYPES
#define ID_LAYER 0u
#define ID_BOX 1u
#define ID_SPHERE 2u
#define ID_CYLINDER 3u
#define ID_CONE 4u
#define ID_TORUS 5u

// OPERATIONS
#define ID_UNION 0u
#define ID_INTER 1u
#define ID_DIFF 2u

#define BOUNDING_EXTRA vec3(0.001,0.001,0.001)

struct Camera{
    vec4 pos;
    vec4 front;
    vec4 right;
    vec4 up;
};
struct Ray{
    vec3 pos;
    vec3 dir;
};
struct Brick{
    uint type;
    uint op;
    vec4 pos;
    vec4 size;
    vec4 rot;
    vec4 color;
    int nextP;
    int child_p;
};
struct sdCap{
    float dist;
    uint index;
    vec4 color;
};

out vec4 FragColor;
in vec2 vTexCoord;

uniform Camera uCamera;
uniform usampler2D uBrickTex;
uniform uint uBrickTexWidth;
uniform Brick uBricks[100];

// types
float sdSphere(vec3 pos, vec3 size){
    return length(pos) - size.x/2.;
}
float sdBox(vec3 pos, vec3 size) {

    vec3 d = abs(pos) - size/2.;
    return max(d.x,max(d.y,d.z));
}
float sdTorus(vec3 pos, vec3 size )
{
    size /= 4;
    return length( vec2(length(pos.xz)-size.x,pos.y) )-size.y;
}
float sdCone(vec3 pos, vec3 size)
{
    pos.y -= size.y/2;
    size.z = size.z*2;
    vec2 q = size.y*vec2(size.x,-size.z)/size.z;
    vec2 w = vec2( length(pos.xz), pos.y );
    
	vec2 a = w - q*clamp( dot(w,q)/dot(q,q), 0.0, 1.0 );
    vec2 b = w - q*vec2( clamp( w.x/q.x, 0.0, 1.0 ), 1.0 );
    float k = sign( q.y );
    float d = min(dot( a, a ),dot(b, b));
    float s = max( k*(w.x*q.y-w.y*q.x),k*(w.y-q.y)  );
	return sqrt(d)*sign(s);
}
float sdCapsule(vec3 pos, vec3 size){
    pos.y += size.y/2;
    pos.y -= clamp( pos.y, 0.0, size.y);
    return sdSphere(pos, size);
}
float sdPlane(vec3 pos){
    return pos.y;
}
float sdCylinder(vec3 pos, vec3 size )
{
    float d = sdCapsule(pos, size);
    pos.y += size.y/2;
    d = max(d, -sdPlane(pos)); // bottom capped
    d = max(d, -sdPlane(-pos + vec3(0,size.y,0))); // top capped
    return d;
}
float sdEllipsoid(vec3 pos, vec3 size)
{
    size = size/2.;
    return (length( pos/size ) - 1.0) * min(min(size.x,size.y),size.z);
}

// ops
sdCap sdUnion(sdCap d1, sdCap d2){
    if (d1.dist < d2.dist)
        return d1;
    return d2;
}
sdCap sdInter(sdCap d1, sdCap d2){
    if (d2.dist < d1.dist)
        return d1;
    return d2;
}
sdCap sdDiff(sdCap d1, sdCap d2){
    d2.dist = -d2.dist;
    return sdInter(d1,d2);
}
float smin(float a, float b, float k) {
    float h = clamp(0.5 + 0.5*(a-b)/k, 0.0, 1.0);
    return mix(a, b, h) - k*h*(1.0-h);
}
sdCap sdSmoothUnion(sdCap d1, sdCap d2, float k){
    vec4 color = mix(vec4(k), d1.color, d2.color);
    return sdCap(smin(d1.dist, d2.dist, k), d1.dist < d2.dist ? d1.index : d2.index, color);
}
sdCap sdBlend(sdCap d1, sdCap d2, float k){
    vec4 color = mix(vec4(k), d1.color, d2.color);
    return sdCap((d1.dist*k) + ((1.-k)*d2.dist), k >= 0.5 ? d1.index : d2.index, color);
}
sdCap sdShell(sdCap d, float k){
    d.dist = abs(d.dist) - (d.dist > 0.? 0. : k);
    return d;
}

sdCap sdObject(vec3 point, Brick brick, uint brickid){
    switch(brick.type){
    case ID_BOX:
        return sdCap(sdBox(point, brick.size.xyz), brickid, brick.color);
    case ID_SPHERE:
        return sdCap(sdEllipsoid(point, brick.size.xyz), brickid, brick.color);
    case ID_CYLINDER:
        return sdCap(sdCylinder(point, brick.size.xyz), brickid, brick.color);
    case ID_CONE:
        return sdCap(sdCone(point, brick.size.xyz), brickid, brick.color);
    case ID_TORUS:
        return sdCap(sdTorus(point, brick.size.xyz), brickid, brick.color);
    }
    return sdCap(MAX_DIST, 0u, brick.color);
}

sdCap sdOp(sdCap current, sdCap value, uint op){
    switch(op){
    case ID_UNION:
        return sdUnion(current, value);
    case ID_DIFF:
        return sdDiff(current, value);
    case ID_INTER:
        return sdInter(current, value);
    }
    return MISS_sd;
}

sdCap sdLayer4(vec3 point, int brick_p){
    sdCap result = MISS_sd;
    while (brick_p >= 0){
        Brick brick = uBricks[brick_p];
        sdCap cap;

        // change point to position and rotation of brick
        vec3 brick_point = point - brick.pos.xyz;
        brick_point = brick_point + 2.0*cross(brick.rot.xyz, cross(brick.rot.xyz, brick_point) + brick.rot.w * brick_point);

        float bounding = sdBox(brick_point, (brick.size.xyz + BOUNDING_EXTRA)*brick.size.w);
        if (bounding >= result.dist){
            brick_p = brick.nextP;
            continue;
        }
        // switch (brick.type){
        // case ID_LAYER:
        //     brick_point /= brick.size.w;

        //     cap = sdLayer2(brick_point, brick.child_p);
        //     cap.dist *= brick.size.w;
        //     break;
        // default:
            cap = sdObject(brick_point, brick, uint(brick_p));
        //     break;
        // }
        result = sdOp(result, cap, brick.op);
        brick_p = brick.nextP;
    }
    return result;
}

sdCap sdLayer3(vec3 point, int brick_p){
    sdCap result = MISS_sd;
    while (brick_p >= 0){
        Brick brick = uBricks[brick_p];
        sdCap cap;

        // change point to position and rotation of brick
        vec3 brick_point = point - brick.pos.xyz;
        brick_point = brick_point + 2.0*cross(brick.rot.xyz, cross(brick.rot.xyz, brick_point) + brick.rot.w * brick_point);

        float bounding = sdBox(brick_point, (brick.size.xyz + BOUNDING_EXTRA)*brick.size.w);
        if (bounding >= result.dist){
            brick_p = brick.nextP;
            continue;
        }
        switch (brick.type){
        case ID_LAYER:
            brick_point /= brick.size.w;

            cap = sdLayer4(brick_point, brick.child_p);
            cap.dist *= brick.size.w;
            break;
        default:
            cap = sdObject(brick_point, brick, uint(brick_p));
            break;
        }
        result = sdOp(result, cap, brick.op);
        brick_p = brick.nextP;
    }
    return result;
}

sdCap sdLayer2(vec3 point, int brick_p){
    sdCap result = MISS_sd;
    while (brick_p >= 0){
        Brick brick = uBricks[brick_p];
        sdCap cap;

        // change point to position and rotation of brick
        vec3 brick_point = point - brick.pos.xyz;
        brick_point = brick_point + 2.0*cross(brick.rot.xyz, cross(brick.rot.xyz, brick_point) + brick.rot.w * brick_point);

        float bounding = sdBox(brick_point, (brick.size.xyz + BOUNDING_EXTRA)*brick.size.w);
        if (bounding >= result.dist){
            brick_p = brick.nextP;
            continue;
        }
        switch (brick.type){
        case ID_LAYER:
            brick_point /= brick.size.w;

            cap = sdLayer3(brick_point, brick.child_p);
            cap.dist *= brick.size.w;
            break;
        default:
            cap = sdObject(brick_point, brick, uint(brick_p));
            break;
        }
        result = sdOp(result, cap, brick.op);
        brick_p = brick.nextP;
    }
    return result;
}

sdCap sdLayer(vec3 point, int brick_p){
    sdCap result = MISS_sd;
    while (brick_p >= 0){
        Brick brick = uBricks[brick_p];
        sdCap cap;

        // change point to position and rotation of brick
        vec3 brick_point = point - brick.pos.xyz;
        brick_point = brick_point + 2.0*cross(brick.rot.xyz, cross(brick.rot.xyz, brick_point) + brick.rot.w * brick_point);

        float bounding = sdBox(brick_point, (brick.size.xyz + BOUNDING_EXTRA)*brick.size.w);
        if (bounding >= result.dist){
            brick_p = brick.nextP;
            continue;
        }
        switch (brick.type){
        case ID_LAYER:
            brick_point /= brick.size.w;

            cap = sdLayer2(brick_point, brick.child_p);
            cap.dist *= brick.size.w;
            break;
        default:
            cap = sdObject(brick_point, brick, uint(brick_p));
            break;
        }
        result = sdOp(result, cap, brick.op);
        brick_p = brick.nextP;
    }
    return result;
}

#define MAX_STACK_ITEMS 10

struct StackItem{
    int brick_p;
    sdCap result;
    vec3 point;
    uint layer_id;
};
StackItem STACK[MAX_STACK_ITEMS];
int stack_p = 0;

StackItem stack_pop(){
    stack_p -= 1;
    return STACK[stack_p];
}
void stack_push(StackItem item){
    if (stack_p < MAX_STACK_ITEMS){
        STACK[stack_p] = item;
        stack_p += 1;
    }
}
bool stack_hasitems(){
    return stack_p > 0;
}
sdCap sdLayer_iter(vec3 point){
    StackItem cs = StackItem(0, MISS_sd, point, 0u); // current state

    while(true){
        bool layer_pushed = false;

        while (cs.brick_p >= 0){
            Brick brick = uBricks[cs.brick_p];
            sdCap cap;
            if (brick.type == ID_LAYER){
                vec3 layer_point = cs.point - brick.pos.xyz;
                layer_point = layer_point + 2.0*cross(brick.rot.xyz, cross(brick.rot.xyz, layer_point) + brick.rot.w * layer_point);
                layer_point /= brick.size.x;
                uint layer_id = uint(cs.brick_p);

                cs.brick_p = brick.nextP; // advance brick for when returning
                stack_push(cs);

                // reset state to layer
                cs.layer_id = layer_id;
                cs.point = layer_point;
                cs.brick_p = brick.child_p;
                cs.result = MISS_sd;
                layer_pushed = true;
                break;
            }else{
                cap = sdObject(cs.point, brick, uint(cs.brick_p));
                cs.result = sdOp(cs.result, cap, brick.op);
                cs.brick_p = brick.nextP;
            }
        }

        if (!layer_pushed){
            // layer done

            if (!stack_hasitems()){
                // last layer finished, return
                return cs.result;
            } else{
                Brick layer = uBricks[cs.layer_id];
                cs.result.dist *= layer.size.x;   // apply sizing
                sdCap last_layer_result = cs.result;
                cs = stack_pop();
                cs.result = sdOp(cs.result, last_layer_result, layer.op);
            }
        }
    }
}
sdCap sdScene(vec3 point){

    // // symmetrie
    // point.x = abs(point.x);
    // point -= vec3(0,0,0);

    // vec3 spherepoint = point - vec3(0,0,4);
    // sdCap sph = sdCap(sdSphere( spherepoint, vec3(1.,0,0)), 0u);
    // vec3 boxPoint = point - vec3(0,3,2);
    // sdCap cub = sdCap(sdBox(boxPoint, vec3(1,1,1)), 1u);
    // cub = sdShell(cub, 0.01);
    // sdCap pla = sdCap(sdPlane(point - vec3(0,-2,0)), 2u);
    // sdCap torus = sdCap(sdTorus(point - vec3(0,-2,2), vec3(4,1.,1)), 3u);
    // sdCap cone = sdCap(sdCone(point - vec3(0,5,2), vec3(0.5,2,2.)), 4u);
    // sdCap cylinder = sdCap(sdCylinder(point - vec3(-3,0,2), vec3(0.5,2,0)), 5u);
    // sdCap capsule = sdCap(sdCapsule(point - vec3(0,0,2), vec3(0.5,2,0.3)), 6u);
    // sdCap elipse = sdCap(sdEllipsoid(point - vec3(2,0,2), vec3(2,1,2)), 6u);
    // result = sdUnion(cub, sph);
    // result = sdUnion(result, pla);
    // res = sdUnion(res, cone);
    // res = sdUnion(res, cylinder);
    // res = sdUnion(res, capsule);
    // res = sdUnion(res, elipse);
    // res = sdDiff(res, torus);

    // Brick current = getBrick(1u);
    // sdCap sd = sdObject(point, current);
    // result = sdOperation(result, sd, current.op);
    // while(current.nextP != 0u){
    //     current = getBrick(current.nextP);
    //     sd = sdObject(point, current);
    //     result = sdOperation(result, sd, current.op);
    // }

    // vec3 spherepoint = point - vec3(0,0,0);
    // sdCap sph = sdCap(sdSphere( spherepoint, vec3(1,0,0)), 0u);
    // result = sdUnion(result, sph);

    return sdLayer(point, 0);
}

sdCap rayMarch(Ray ray){
    float dist = 0.;
    
    for (uint i = 0u; i < MAX_STEPS; ++i){
        sdCap sd = sdScene(ray.pos + dist*ray.dir);

        dist += sd.dist;

        if (sd.dist < SURFACE_DIST)
            return sdCap(dist, sd.index, sd.color);

        if (sd.dist > MAX_DIST)
            break;
    }

    return MISS_sd;
}

vec3 getNormal(vec3 pos){
    sdCap d = sdScene(pos);
    
    vec2 e = vec2(0.001,0);
    
    vec3 normal = d.dist - vec3(
        sdScene(pos-e.xyy).dist,
        sdScene(pos-e.yxy).dist,
        sdScene(pos-e.yyx).dist
    );

    return normalize(normal);
}

vec4 pointColor(vec3 pos){

    vec3 normal = getNormal(pos);
    vec3 toLight = normalize(lightpos - pos);
    float diffuse = clamp(dot(normal, toLight), 0.1, 1.);

    Ray ray = Ray(pos + normal*SURFACE_DIST*2., toLight);
    sdCap lightHit = rayMarch(ray);

    if (lightHit.dist < length(lightpos-pos)) diffuse *= 0.5;

    return vec4(vec3(diffuse),1.);
}

vec4 getColor(Ray ray){
    vec4 color = vec4(0,0,0,0);

    float mask = 1.;
    for (uint i = 0u; i < MAX_RAYS;++i){
        sdCap rayhit = rayMarch(ray);
        if (rayhit.dist > MAX_DIST){
            if (i < 1u)
                color = vec4(0,0,0,0);
            break;
        }

        vec3 hitpos = ray.pos + rayhit.dist*ray.dir;
        color += rayhit.color*mask*pointColor(hitpos);

        ray.dir = reflect(ray.dir, getNormal(hitpos));
        ray.pos = hitpos + ray.dir*SURFACE_DIST*2.;
        mask *= 0.2;
    }
    
    return color;
}


Ray createRay(vec2 coord, Camera camera){
    vec2 uv = coord - vec2(0.5,0.5);
    vec4 dir = normalize(camera.front + uv.x*camera.right + uv.y*camera.up);
    return Ray(camera.pos.xyz, dir.xyz);
}

void main(){
    vec2 uv = vTexCoord;

    Ray ray = createRay(uv, uCamera);
    FragColor = getColor(ray);
}