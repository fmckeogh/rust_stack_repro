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
use execute_aarch32_instrs_VTBL_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VTBL_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    Vn: u8,
    Vd: u8,
    len: u8,
    N: bool,
    op: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        length: i64,
        is_vtbl: bool,
        n: i64,
        d: i64,
        D: bool,
        Vn: u8,
        Vd: u8,
        len: u8,
        N: bool,
        op: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        D,
        Vn,
        Vd,
        len,
        N,
        op,
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
        // D s_2_0: read-var op:u8
        let s_2_0: bool = fn_state.op;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #0u : u8
        let s_2_2: bool = false;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // D s_2_5: write-var is_vtbl <= s_2_4
        fn_state.is_vtbl = s_2_4;
        // D s_2_6: read-var len:u8
        let s_2_6: u8 = fn_state.len;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 2u16);
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (s_2_7.value() as i128);
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // C s_2_10: const #1s : i
        let s_2_10: i128 = 1;
        // D s_2_11: cast zx s_2_9 -> i
        let s_2_11: i128 = (i128::try_from(s_2_9).unwrap());
        // D s_2_12: add s_2_11 s_2_10
        let s_2_12: i128 = (s_2_11 + s_2_10);
        // D s_2_13: cast reint s_2_12 -> i64
        let s_2_13: i64 = (s_2_12 as i64);
        // D s_2_14: write-var length <= s_2_13
        fn_state.length = s_2_13;
        // D s_2_15: read-var D:u8
        let s_2_15: bool = fn_state.D;
        // D s_2_16: cast zx s_2_15 -> bv
        let s_2_16: Bits = Bits::new(s_2_15 as u128, 1u16);
        // D s_2_17: read-var Vd:u8
        let s_2_17: u8 = fn_state.Vd;
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 4u16);
        // D s_2_19: cast reint s_2_16 -> u128
        let s_2_19: u128 = (s_2_16.value() as u128);
        // D s_2_20: size-of s_2_16
        let s_2_20: u16 = s_2_16.length();
        // D s_2_21: cast reint s_2_18 -> u128
        let s_2_21: u128 = (s_2_18.value() as u128);
        // D s_2_22: size-of s_2_18
        let s_2_22: u16 = s_2_18.length();
        // D s_2_23: lsl s_2_19 s_2_22
        let s_2_23: u128 = s_2_19 << s_2_22;
        // D s_2_24: or s_2_23 s_2_21
        let s_2_24: u128 = ((s_2_23) | (s_2_21));
        // D s_2_25: add s_2_20 s_2_22
        let s_2_25: u16 = (s_2_20 + s_2_22);
        // D s_2_26: create-bits s_2_24 s_2_25
        let s_2_26: Bits = Bits::new(s_2_24, s_2_25);
        // D s_2_27: cast reint s_2_26 -> u8
        let s_2_27: u8 = (s_2_26.value() as u8);
        // D s_2_28: cast zx s_2_27 -> bv
        let s_2_28: Bits = Bits::new(s_2_27 as u128, 5u16);
        // D s_2_29: cast zx s_2_28 -> i
        let s_2_29: i128 = (s_2_28.value() as i128);
        // D s_2_30: cast reint s_2_29 -> i64
        let s_2_30: i64 = (s_2_29 as i64);
        // D s_2_31: write-var d <= s_2_30
        fn_state.d = s_2_30;
        // D s_2_32: read-var N:u8
        let s_2_32: bool = fn_state.N;
        // D s_2_33: cast zx s_2_32 -> bv
        let s_2_33: Bits = Bits::new(s_2_32 as u128, 1u16);
        // D s_2_34: read-var Vn:u8
        let s_2_34: u8 = fn_state.Vn;
        // D s_2_35: cast zx s_2_34 -> bv
        let s_2_35: Bits = Bits::new(s_2_34 as u128, 4u16);
        // D s_2_36: cast reint s_2_33 -> u128
        let s_2_36: u128 = (s_2_33.value() as u128);
        // D s_2_37: size-of s_2_33
        let s_2_37: u16 = s_2_33.length();
        // D s_2_38: cast reint s_2_35 -> u128
        let s_2_38: u128 = (s_2_35.value() as u128);
        // D s_2_39: size-of s_2_35
        let s_2_39: u16 = s_2_35.length();
        // D s_2_40: lsl s_2_36 s_2_39
        let s_2_40: u128 = s_2_36 << s_2_39;
        // D s_2_41: or s_2_40 s_2_38
        let s_2_41: u128 = ((s_2_40) | (s_2_38));
        // D s_2_42: add s_2_37 s_2_39
        let s_2_42: u16 = (s_2_37 + s_2_39);
        // D s_2_43: create-bits s_2_41 s_2_42
        let s_2_43: Bits = Bits::new(s_2_41, s_2_42);
        // D s_2_44: cast reint s_2_43 -> u8
        let s_2_44: u8 = (s_2_43.value() as u8);
        // D s_2_45: cast zx s_2_44 -> bv
        let s_2_45: Bits = Bits::new(s_2_44 as u128, 5u16);
        // D s_2_46: cast zx s_2_45 -> i
        let s_2_46: i128 = (s_2_45.value() as i128);
        // D s_2_47: cast reint s_2_46 -> i64
        let s_2_47: i64 = (s_2_46 as i64);
        // D s_2_48: write-var n <= s_2_47
        fn_state.n = s_2_47;
        // D s_2_49: read-var M:u8
        let s_2_49: bool = fn_state.M;
        // D s_2_50: cast zx s_2_49 -> bv
        let s_2_50: Bits = Bits::new(s_2_49 as u128, 1u16);
        // D s_2_51: read-var Vm:u8
        let s_2_51: u8 = fn_state.Vm;
        // D s_2_52: cast zx s_2_51 -> bv
        let s_2_52: Bits = Bits::new(s_2_51 as u128, 4u16);
        // D s_2_53: cast reint s_2_50 -> u128
        let s_2_53: u128 = (s_2_50.value() as u128);
        // D s_2_54: size-of s_2_50
        let s_2_54: u16 = s_2_50.length();
        // D s_2_55: cast reint s_2_52 -> u128
        let s_2_55: u128 = (s_2_52.value() as u128);
        // D s_2_56: size-of s_2_52
        let s_2_56: u16 = s_2_52.length();
        // D s_2_57: lsl s_2_53 s_2_56
        let s_2_57: u128 = s_2_53 << s_2_56;
        // D s_2_58: or s_2_57 s_2_55
        let s_2_58: u128 = ((s_2_57) | (s_2_55));
        // D s_2_59: add s_2_54 s_2_56
        let s_2_59: u16 = (s_2_54 + s_2_56);
        // D s_2_60: create-bits s_2_58 s_2_59
        let s_2_60: Bits = Bits::new(s_2_58, s_2_59);
        // D s_2_61: cast reint s_2_60 -> u8
        let s_2_61: u8 = (s_2_60.value() as u8);
        // D s_2_62: cast zx s_2_61 -> bv
        let s_2_62: Bits = Bits::new(s_2_61 as u128, 5u16);
        // D s_2_63: cast zx s_2_62 -> i
        let s_2_63: i128 = (s_2_62.value() as i128);
        // D s_2_64: cast reint s_2_63 -> i64
        let s_2_64: i64 = (s_2_63 as i64);
        // D s_2_65: write-var m <= s_2_64
        fn_state.m = s_2_64;
        // D s_2_66: read-var n:i64
        let s_2_66: i64 = fn_state.n;
        // D s_2_67: cast zx s_2_66 -> i
        let s_2_67: i128 = (i128::try_from(s_2_66).unwrap());
        // D s_2_68: read-var length:i64
        let s_2_68: i64 = fn_state.length;
        // D s_2_69: cast zx s_2_68 -> i
        let s_2_69: i128 = (i128::try_from(s_2_68).unwrap());
        // D s_2_70: add s_2_67 s_2_69
        let s_2_70: i128 = (s_2_67 + s_2_69);
        // D s_2_71: cast reint s_2_70 -> i64
        let s_2_71: i64 = (s_2_70 as i64);
        // C s_2_72: const #32s : i
        let s_2_72: i128 = 32;
        // D s_2_73: cast zx s_2_71 -> i
        let s_2_73: i128 = (i128::try_from(s_2_71).unwrap());
        // D s_2_74: cmp-gt s_2_73 s_2_72
        let s_2_74: bool = ((s_2_73) > (s_2_72));
        // N s_2_75: branch s_2_74 b4 b3
        if s_2_74 {
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
        // D s_3_0: read-var d:i64
        let s_3_0: i64 = fn_state.d;
        // D s_3_1: read-var is_vtbl:u8
        let s_3_1: bool = fn_state.is_vtbl;
        // D s_3_2: read-var length:i64
        let s_3_2: i64 = fn_state.length;
        // D s_3_3: read-var m:i64
        let s_3_3: i64 = fn_state.m;
        // D s_3_4: read-var n:i64
        let s_3_4: i64 = fn_state.n;
        // D s_3_5: call execute_aarch32_instrs_VTBL_Op_A_txt(s_3_0, s_3_1, s_3_2, s_3_3, s_3_4)
        let s_3_5: () = execute_aarch32_instrs_VTBL_Op_A_txt(
            state,
            tracer,
            s_3_0,
            s_3_1,
            s_3_2,
            s_3_3,
            s_3_4,
        );
        // N s_3_6: return
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
