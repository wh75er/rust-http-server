use crate::person::*;

#[test]
fn correct_validation_test() {
    let p = Person {
        id: 0,
        age: 4,
        name: "test".to_string(),
        work: "test".to_string(),
        address: "test".to_string(),
    };

    let val = p.validate();
    
    assert_eq!(true, val.is_ok());
}

#[test]
fn incorrect_age_validation_test() {
    let p = Person {
        id: 0,
        age: -1,
        name: "test".to_string(),
        work: "test".to_string(),
        address: "test".to_string(),
    };

    let val = p.validate();

    assert_eq!(false, val.is_ok());
    
    let mut verr = val.err().unwrap();

    assert_eq!(1, verr.len());
    assert_eq!(ValidateErr::AgeErr, verr.pop().unwrap());
}

#[test]
fn empty_name_validation_test() {
    let p = Person {
        id: 0,
        age: 4,
        name: "".to_string(),
        work: "test".to_string(),
        address: "test".to_string(),
    };

    let val = p.validate();

    assert_eq!(false, val.is_ok());

    let mut verr = val.err().unwrap();

    assert_eq!(1, verr.len());
    assert_eq!(ValidateErr::NameErr, verr.pop().unwrap());
}

#[test]
fn empty_work_validation_test() {
    let p = Person {
        id: 0,
        age: 4,
        name: "test".to_string(),
        work: "".to_string(),
        address: "test".to_string(),
    };

    let val = p.validate();

    assert_eq!(false, val.is_ok());

    let mut verr = val.err().unwrap();

    assert_eq!(1, verr.len());
    assert_eq!(ValidateErr::WorkErr, verr.pop().unwrap());
}

#[test]
fn empty_address_validation_test() {
    let p = Person {
        id: 0,
        age: 4,
        name: "test".to_string(),
        work: "test".to_string(),
        address: "".to_string(),
    };

    let val = p.validate();

    assert_eq!(false, val.is_ok());

    let mut verr = val.err().unwrap();

    assert_eq!(1, verr.len());
    assert_eq!(ValidateErr::AddressErr, verr.pop().unwrap());
}
