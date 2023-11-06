#version 330 core


in vec2 vTexCoord;
in vec3 vNormal;
in vec3 vFragPos;

uniform sampler2D uTexture;
uniform vec4 uColor;

void main()
{
	// gl_FragColor = texture2D(uTexture, vTexCoord);
	// gl_FragColor = vec4(vTexCoord,0.,1.0);
	// gl_FragColor = vec4(abs(vNormal), 1);
	if (uColor.w > 0.){
		vec3 lightColor = vec3(1.,1.,1.);
		vec3 lightDir = normalize(-vec3(0.,-1.,1.));

		// calc ambient
		float ambientStrength = 0.1;
		vec3 ambient = ambientStrength * lightColor;

		// calc diffurse
		vec3 norm = normalize(vNormal); 
		float diff = max(dot(norm, lightDir), 0.0);
		vec3 diffuse = diff * lightColor;

		// calc result
		vec3 result = (ambient + diffuse) * vec3(uColor);
		gl_FragColor = vec4(result, 1.0);
	}
	else{
		gl_FragColor = vec4(vTexCoord,0.,1.0);
	}
}