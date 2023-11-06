use crate::Vecf3;



#[test]
fn test_angle_between(){
    assert_eq!(Vecf3::new(0.,1.,0.).angle_between(&Vecf3::new(1.,0.,0.)).to_degrees(), 90.);
}

#[test]
fn test_norm(){
    assert_eq!(Vecf3::new(0.,0.5,0.).norm(), Vecf3::new(0., 1., 0.));
    assert_eq!(Vecf3::new(0.,-0.5,0.).norm(), Vecf3::new(0., -1., 0.));
    assert_eq!(Vecf3::new(0.,0.,0.).norm(), Vecf3::new(0., 0., 0.));
}