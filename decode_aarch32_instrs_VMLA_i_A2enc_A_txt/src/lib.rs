#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use ConditionPassed::*;
use execute_aarch32_instrs_VMLA_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VMLA_i_A2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    U: bool,
    D: bool,
    size: u8,
    Vn: u8,
    Vd: u8,
    op: bool,
    N: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        U: bool,
        D: bool,
        size: u8,
        Vn: u8,
        Vd: u8,
        op: bool,
        N: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        U,
        D,
        size,
        Vn,
        Vd,
        op,
        N,
        M,
        Vm,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call ConditionPassed(s_0_0)
        let s_0_1: bool = ConditionPassed(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var size:u8
        let s_2_0: u8 = fn_state.size;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #3u : u8
        let s_2_2: u8 = 3;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b6 b3
        if s_2_4 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i
        let s_3_0: i128 = 0;
        // D s_3_1: read-var Vd:u8
        let s_3_1: u8 = fn_state.Vd;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 4u16);
        // C s_3_3: const #1u : u64
        let s_3_3: u64 = 1;
        // D s_3_4: bit-extract s_3_2 s_3_0 s_3_3
        let s_3_4: Bits = (Bits::new(
            ((s_3_2) >> (s_3_0)).value(),
            u16::try_from(s_3_3).unwrap(),
        ));
        // D s_3_5: cast reint s_3_4 -> u8
        let s_3_5: bool = ((s_3_4.value()) != 0);
        // C s_3_6: const #0s : i
        let s_3_6: i128 = 0;
        // C s_3_7: const #0u : u64
        let s_3_7: u64 = 0;
        // D s_3_8: cast zx s_3_5 -> u64
        let s_3_8: u64 = (s_3_5 as u64);
        // C s_3_9: const #1u : u64
        let s_3_9: u64 = 1;
        // D s_3_10: and s_3_8 s_3_9
        let s_3_10: u64 = ((s_3_8) & (s_3_9));
        // D s_3_11: cmp-eq s_3_10 s_3_9
        let s_3_11: bool = ((s_3_10) == (s_3_9));
        // D s_3_12: lsl s_3_8 s_3_6
        let s_3_12: u64 = s_3_8 << s_3_6;
        // D s_3_13: or s_3_7 s_3_12
        let s_3_13: u64 = ((s_3_7) | (s_3_12));
        // D s_3_14: cmpl s_3_12
        let s_3_14: u64 = !s_3_12;
        // D s_3_15: and s_3_7 s_3_14
        let s_3_15: u64 = ((s_3_7) & (s_3_14));
        // D s_3_16: select s_3_11 s_3_13 s_3_15
        let s_3_16: u64 = if s_3_11 { s_3_13 } else { s_3_15 };
        // D s_3_17: cast trunc s_3_16 -> u8
        let s_3_17: bool = ((s_3_16) != 0);
        // D s_3_18: cast zx s_3_17 -> bv
        let s_3_18: Bits = Bits::new(s_3_17 as u128, 1u16);
        // C s_3_19: const #1u : u8
        let s_3_19: bool = true;
        // C s_3_20: cast zx s_3_19 -> bv
        let s_3_20: Bits = Bits::new(s_3_19 as u128, 1u16);
        // D s_3_21: cmp-eq s_3_18 s_3_20
        let s_3_21: bool = ((s_3_18) == (s_3_20));
        // N s_3_22: branch s_3_21 b5 b4
        if s_3_21 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var op:u8
        let s_4_0: bool = fn_state.op;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 1u16);
        // C s_4_2: const #0u : u8
        let s_4_2: bool = false;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // D s_4_5: read-var U:u8
        let s_4_5: bool = fn_state.U;
        // D s_4_6: cast zx s_4_5 -> bv
        let s_4_6: Bits = Bits::new(s_4_5 as u128, 1u16);
        // C s_4_7: const #1u : u8
        let s_4_7: bool = true;
        // C s_4_8: cast zx s_4_7 -> bv
        let s_4_8: Bits = Bits::new(s_4_7 as u128, 1u16);
        // D s_4_9: cmp-eq s_4_6 s_4_8
        let s_4_9: bool = ((s_4_6) == (s_4_8));
        // D s_4_10: read-var size:u8
        let s_4_10: u8 = fn_state.size;
        // D s_4_11: cast zx s_4_10 -> bv
        let s_4_11: Bits = Bits::new(s_4_10 as u128, 2u16);
        // D s_4_12: cast zx s_4_11 -> i
        let s_4_12: i128 = (s_4_11.value() as i128);
        // D s_4_13: cast reint s_4_12 -> i64
        let s_4_13: i64 = (s_4_12 as i64);
        // C s_4_14: const #8s : i64
        let s_4_14: i64 = 8;
        // D s_4_15: lsl s_4_14 s_4_13
        let s_4_15: i64 = s_4_14 << s_4_13;
        // C s_4_16: const #64s : i
        let s_4_16: i128 = 64;
        // D s_4_17: cast zx s_4_15 -> i
        let s_4_17: i128 = (i128::try_from(s_4_15).unwrap());
        // D s_4_18: div s_4_16 s_4_17
        let s_4_18: i128 = ((s_4_16) / (s_4_17));
        // D s_4_19: cast reint s_4_18 -> i64
        let s_4_19: i64 = (s_4_18 as i64);
        // D s_4_20: read-var D:u8
        let s_4_20: bool = fn_state.D;
        // D s_4_21: cast zx s_4_20 -> bv
        let s_4_21: Bits = Bits::new(s_4_20 as u128, 1u16);
        // D s_4_22: read-var Vd:u8
        let s_4_22: u8 = fn_state.Vd;
        // D s_4_23: cast zx s_4_22 -> bv
        let s_4_23: Bits = Bits::new(s_4_22 as u128, 4u16);
        // D s_4_24: cast reint s_4_21 -> u128
        let s_4_24: u128 = (s_4_21.value() as u128);
        // D s_4_25: size-of s_4_21
        let s_4_25: u16 = s_4_21.length();
        // D s_4_26: cast reint s_4_23 -> u128
        let s_4_26: u128 = (s_4_23.value() as u128);
        // D s_4_27: size-of s_4_23
        let s_4_27: u16 = s_4_23.length();
        // D s_4_28: lsl s_4_24 s_4_27
        let s_4_28: u128 = s_4_24 << s_4_27;
        // D s_4_29: or s_4_28 s_4_26
        let s_4_29: u128 = ((s_4_28) | (s_4_26));
        // D s_4_30: add s_4_25 s_4_27
        let s_4_30: u16 = (s_4_25 + s_4_27);
        // D s_4_31: create-bits s_4_29 s_4_30
        let s_4_31: Bits = Bits::new(s_4_29, s_4_30);
        // D s_4_32: cast reint s_4_31 -> u8
        let s_4_32: u8 = (s_4_31.value() as u8);
        // D s_4_33: cast zx s_4_32 -> bv
        let s_4_33: Bits = Bits::new(s_4_32 as u128, 5u16);
        // D s_4_34: cast zx s_4_33 -> i
        let s_4_34: i128 = (s_4_33.value() as i128);
        // D s_4_35: cast reint s_4_34 -> i64
        let s_4_35: i64 = (s_4_34 as i64);
        // D s_4_36: read-var N:u8
        let s_4_36: bool = fn_state.N;
        // D s_4_37: cast zx s_4_36 -> bv
        let s_4_37: Bits = Bits::new(s_4_36 as u128, 1u16);
        // D s_4_38: read-var Vn:u8
        let s_4_38: u8 = fn_state.Vn;
        // D s_4_39: cast zx s_4_38 -> bv
        let s_4_39: Bits = Bits::new(s_4_38 as u128, 4u16);
        // D s_4_40: cast reint s_4_37 -> u128
        let s_4_40: u128 = (s_4_37.value() as u128);
        // D s_4_41: size-of s_4_37
        let s_4_41: u16 = s_4_37.length();
        // D s_4_42: cast reint s_4_39 -> u128
        let s_4_42: u128 = (s_4_39.value() as u128);
        // D s_4_43: size-of s_4_39
        let s_4_43: u16 = s_4_39.length();
        // D s_4_44: lsl s_4_40 s_4_43
        let s_4_44: u128 = s_4_40 << s_4_43;
        // D s_4_45: or s_4_44 s_4_42
        let s_4_45: u128 = ((s_4_44) | (s_4_42));
        // D s_4_46: add s_4_41 s_4_43
        let s_4_46: u16 = (s_4_41 + s_4_43);
        // D s_4_47: create-bits s_4_45 s_4_46
        let s_4_47: Bits = Bits::new(s_4_45, s_4_46);
        // D s_4_48: cast reint s_4_47 -> u8
        let s_4_48: u8 = (s_4_47.value() as u8);
        // D s_4_49: cast zx s_4_48 -> bv
        let s_4_49: Bits = Bits::new(s_4_48 as u128, 5u16);
        // D s_4_50: cast zx s_4_49 -> i
        let s_4_50: i128 = (s_4_49.value() as i128);
        // D s_4_51: cast reint s_4_50 -> i64
        let s_4_51: i64 = (s_4_50 as i64);
        // D s_4_52: read-var M:u8
        let s_4_52: bool = fn_state.M;
        // D s_4_53: cast zx s_4_52 -> bv
        let s_4_53: Bits = Bits::new(s_4_52 as u128, 1u16);
        // D s_4_54: read-var Vm:u8
        let s_4_54: u8 = fn_state.Vm;
        // D s_4_55: cast zx s_4_54 -> bv
        let s_4_55: Bits = Bits::new(s_4_54 as u128, 4u16);
        // D s_4_56: cast reint s_4_53 -> u128
        let s_4_56: u128 = (s_4_53.value() as u128);
        // D s_4_57: size-of s_4_53
        let s_4_57: u16 = s_4_53.length();
        // D s_4_58: cast reint s_4_55 -> u128
        let s_4_58: u128 = (s_4_55.value() as u128);
        // D s_4_59: size-of s_4_55
        let s_4_59: u16 = s_4_55.length();
        // D s_4_60: lsl s_4_56 s_4_59
        let s_4_60: u128 = s_4_56 << s_4_59;
        // D s_4_61: or s_4_60 s_4_58
        let s_4_61: u128 = ((s_4_60) | (s_4_58));
        // D s_4_62: add s_4_57 s_4_59
        let s_4_62: u16 = (s_4_57 + s_4_59);
        // D s_4_63: create-bits s_4_61 s_4_62
        let s_4_63: Bits = Bits::new(s_4_61, s_4_62);
        // D s_4_64: cast reint s_4_63 -> u8
        let s_4_64: u8 = (s_4_63.value() as u8);
        // D s_4_65: cast zx s_4_64 -> bv
        let s_4_65: Bits = Bits::new(s_4_64 as u128, 5u16);
        // D s_4_66: cast zx s_4_65 -> i
        let s_4_66: i128 = (s_4_65.value() as i128);
        // D s_4_67: cast reint s_4_66 -> i64
        let s_4_67: i64 = (s_4_66 as i64);
        // D s_4_68: cast zx s_4_15 -> i
        let s_4_68: i128 = (i128::try_from(s_4_15).unwrap());
        // D s_4_69: cast reint s_4_68 -> i64
        let s_4_69: i64 = (s_4_68 as i64);
        // D s_4_70: cast zx s_4_19 -> i
        let s_4_70: i128 = (i128::try_from(s_4_19).unwrap());
        // C s_4_71: const #1s : i64
        let s_4_71: i64 = 1;
        // C s_4_72: const #1u : u8
        let s_4_72: bool = true;
        // D s_4_73: call execute_aarch32_instrs_VMLA_i_Op_A_txt(s_4_4, s_4_35, s_4_70, s_4_69, s_4_72, s_4_67, s_4_51, s_4_71, s_4_9)
        let s_4_73: () = execute_aarch32_instrs_VMLA_i_Op_A_txt(
            state,
            tracer,
            s_4_4,
            s_4_35,
            s_4_70,
            s_4_69,
            s_4_72,
            s_4_67,
            s_4_51,
            s_4_71,
            s_4_9,
        );
        // N s_4_74: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: panic
        panic!("{:?}", ());
        // N s_5_1: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: panic
        panic!("{:?}", ());
        // N s_6_1: return
        return;
    }
}
