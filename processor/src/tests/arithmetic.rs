use isa::{FlagIndex, Instruction};

use crate::{
    tests::{exec, setup_processor},
    MAX_VALUE_MEMORY,
};

#[test]
fn test_add_sets_correct_value() {
    let mut p = setup_processor();

    p.set_reg(0, 10).unwrap();
    p.set_rx(0).unwrap();
    p.set_reg(1, 8).unwrap();
    p.set_ry(1).unwrap();
    p.set_reg(2, 2).unwrap();
    p.set_rz(2).unwrap();

    exec(Instruction::ADD, &mut p);

    assert_eq!(p.reg(0).unwrap(), 10);
}

#[test]
fn test_add_sets_corret_value_and_flags_with_overflow() {
    let mut p = setup_processor();

    p.set_reg(0, 2).unwrap();
    p.set_rx(0).unwrap();
    p.set_reg(1, MAX_VALUE_MEMORY).unwrap();
    p.set_ry(1).unwrap();
    p.set_reg(2, 2).unwrap();
    p.set_rz(2).unwrap();

    exec(Instruction::ADD, &mut p);

    assert_eq!(p.reg(0).unwrap(), 2);
    assert!(p.fr(FlagIndex::CARRY).unwrap());
    assert!(p.fr(FlagIndex::ARITHMETIC_OVERFLOW).unwrap())
}
