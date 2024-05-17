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
use execute_aarch32_instrs_VPMAX_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VPMAX_i_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    U: bool,
    D: bool,
    size: u8,
    Vn: u8,
    Vd: u8,
    N: bool,
    M: bool,
    op: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        U: bool,
        D: bool,
        size: u8,
        Vn: u8,
        Vd: u8,
        N: bool,
        M: bool,
        op: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        U,
        D,
        size,
        Vn,
        Vd,
        N,
        M,
        op,
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
        // N s_2_5: branch s_2_4 b4 b3
        if s_2_4 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var op:u8
        let s_3_0: bool = fn_state.op;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // C s_3_2: const #0u : u8
        let s_3_2: bool = false;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // D s_3_5: read-var U:u8
        let s_3_5: bool = fn_state.U;
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 1u16);
        // C s_3_7: const #1u : u8
        let s_3_7: bool = true;
        // C s_3_8: cast zx s_3_7 -> bv
        let s_3_8: Bits = Bits::new(s_3_7 as u128, 1u16);
        // D s_3_9: cmp-eq s_3_6 s_3_8
        let s_3_9: bool = ((s_3_6) == (s_3_8));
        // D s_3_10: read-var size:u8
        let s_3_10: u8 = fn_state.size;
        // D s_3_11: cast zx s_3_10 -> bv
        let s_3_11: Bits = Bits::new(s_3_10 as u128, 2u16);
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (s_3_11.value() as i128);
        // D s_3_13: cast reint s_3_12 -> i64
        let s_3_13: i64 = (s_3_12 as i64);
        // C s_3_14: const #8s : i64
        let s_3_14: i64 = 8;
        // D s_3_15: lsl s_3_14 s_3_13
        let s_3_15: i64 = s_3_14 << s_3_13;
        // C s_3_16: const #64s : i
        let s_3_16: i128 = 64;
        // D s_3_17: cast zx s_3_15 -> i
        let s_3_17: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_18: div s_3_16 s_3_17
        let s_3_18: i128 = ((s_3_16) / (s_3_17));
        // D s_3_19: cast reint s_3_18 -> i64
        let s_3_19: i64 = (s_3_18 as i64);
        // D s_3_20: read-var D:u8
        let s_3_20: bool = fn_state.D;
        // D s_3_21: cast zx s_3_20 -> bv
        let s_3_21: Bits = Bits::new(s_3_20 as u128, 1u16);
        // D s_3_22: read-var Vd:u8
        let s_3_22: u8 = fn_state.Vd;
        // D s_3_23: cast zx s_3_22 -> bv
        let s_3_23: Bits = Bits::new(s_3_22 as u128, 4u16);
        // D s_3_24: cast reint s_3_21 -> u128
        let s_3_24: u128 = (s_3_21.value() as u128);
        // D s_3_25: size-of s_3_21
        let s_3_25: u16 = s_3_21.length();
        // D s_3_26: cast reint s_3_23 -> u128
        let s_3_26: u128 = (s_3_23.value() as u128);
        // D s_3_27: size-of s_3_23
        let s_3_27: u16 = s_3_23.length();
        // D s_3_28: lsl s_3_24 s_3_27
        let s_3_28: u128 = s_3_24 << s_3_27;
        // D s_3_29: or s_3_28 s_3_26
        let s_3_29: u128 = ((s_3_28) | (s_3_26));
        // D s_3_30: add s_3_25 s_3_27
        let s_3_30: u16 = (s_3_25 + s_3_27);
        // D s_3_31: create-bits s_3_29 s_3_30
        let s_3_31: Bits = Bits::new(s_3_29, s_3_30);
        // D s_3_32: cast reint s_3_31 -> u8
        let s_3_32: u8 = (s_3_31.value() as u8);
        // D s_3_33: cast zx s_3_32 -> bv
        let s_3_33: Bits = Bits::new(s_3_32 as u128, 5u16);
        // D s_3_34: cast zx s_3_33 -> i
        let s_3_34: i128 = (s_3_33.value() as i128);
        // D s_3_35: cast reint s_3_34 -> i64
        let s_3_35: i64 = (s_3_34 as i64);
        // D s_3_36: read-var N:u8
        let s_3_36: bool = fn_state.N;
        // D s_3_37: cast zx s_3_36 -> bv
        let s_3_37: Bits = Bits::new(s_3_36 as u128, 1u16);
        // D s_3_38: read-var Vn:u8
        let s_3_38: u8 = fn_state.Vn;
        // D s_3_39: cast zx s_3_38 -> bv
        let s_3_39: Bits = Bits::new(s_3_38 as u128, 4u16);
        // D s_3_40: cast reint s_3_37 -> u128
        let s_3_40: u128 = (s_3_37.value() as u128);
        // D s_3_41: size-of s_3_37
        let s_3_41: u16 = s_3_37.length();
        // D s_3_42: cast reint s_3_39 -> u128
        let s_3_42: u128 = (s_3_39.value() as u128);
        // D s_3_43: size-of s_3_39
        let s_3_43: u16 = s_3_39.length();
        // D s_3_44: lsl s_3_40 s_3_43
        let s_3_44: u128 = s_3_40 << s_3_43;
        // D s_3_45: or s_3_44 s_3_42
        let s_3_45: u128 = ((s_3_44) | (s_3_42));
        // D s_3_46: add s_3_41 s_3_43
        let s_3_46: u16 = (s_3_41 + s_3_43);
        // D s_3_47: create-bits s_3_45 s_3_46
        let s_3_47: Bits = Bits::new(s_3_45, s_3_46);
        // D s_3_48: cast reint s_3_47 -> u8
        let s_3_48: u8 = (s_3_47.value() as u8);
        // D s_3_49: cast zx s_3_48 -> bv
        let s_3_49: Bits = Bits::new(s_3_48 as u128, 5u16);
        // D s_3_50: cast zx s_3_49 -> i
        let s_3_50: i128 = (s_3_49.value() as i128);
        // D s_3_51: cast reint s_3_50 -> i64
        let s_3_51: i64 = (s_3_50 as i64);
        // D s_3_52: read-var M:u8
        let s_3_52: bool = fn_state.M;
        // D s_3_53: cast zx s_3_52 -> bv
        let s_3_53: Bits = Bits::new(s_3_52 as u128, 1u16);
        // D s_3_54: read-var Vm:u8
        let s_3_54: u8 = fn_state.Vm;
        // D s_3_55: cast zx s_3_54 -> bv
        let s_3_55: Bits = Bits::new(s_3_54 as u128, 4u16);
        // D s_3_56: cast reint s_3_53 -> u128
        let s_3_56: u128 = (s_3_53.value() as u128);
        // D s_3_57: size-of s_3_53
        let s_3_57: u16 = s_3_53.length();
        // D s_3_58: cast reint s_3_55 -> u128
        let s_3_58: u128 = (s_3_55.value() as u128);
        // D s_3_59: size-of s_3_55
        let s_3_59: u16 = s_3_55.length();
        // D s_3_60: lsl s_3_56 s_3_59
        let s_3_60: u128 = s_3_56 << s_3_59;
        // D s_3_61: or s_3_60 s_3_58
        let s_3_61: u128 = ((s_3_60) | (s_3_58));
        // D s_3_62: add s_3_57 s_3_59
        let s_3_62: u16 = (s_3_57 + s_3_59);
        // D s_3_63: create-bits s_3_61 s_3_62
        let s_3_63: Bits = Bits::new(s_3_61, s_3_62);
        // D s_3_64: cast reint s_3_63 -> u8
        let s_3_64: u8 = (s_3_63.value() as u8);
        // D s_3_65: cast zx s_3_64 -> bv
        let s_3_65: Bits = Bits::new(s_3_64 as u128, 5u16);
        // D s_3_66: cast zx s_3_65 -> i
        let s_3_66: i128 = (s_3_65.value() as i128);
        // D s_3_67: cast reint s_3_66 -> i64
        let s_3_67: i64 = (s_3_66 as i64);
        // D s_3_68: cast zx s_3_15 -> i
        let s_3_68: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_69: cast reint s_3_68 -> i64
        let s_3_69: i64 = (s_3_68 as i64);
        // D s_3_70: cast zx s_3_19 -> i
        let s_3_70: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_71: call execute_aarch32_instrs_VPMAX_i_Op_A_txt(s_3_35, s_3_70, s_3_69, s_3_67, s_3_4, s_3_51, s_3_9)
        let s_3_71: () = execute_aarch32_instrs_VPMAX_i_Op_A_txt(
            state,
            tracer,
            s_3_35,
            s_3_70,
            s_3_69,
            s_3_67,
            s_3_4,
            s_3_51,
            s_3_9,
        );
        // N s_3_72: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: panic
        panic!("{:?}", ());
        // N s_4_1: return
        return;
    }
}
