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
use neq_int::*;
use ConditionPassed::*;
use execute_aarch32_instrs_VLD1_1_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VLD1_1_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    Rn: u8,
    Vd: u8,
    size: u8,
    index_align: u8,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        n: i64,
        index: i64,
        d: i64,
        gs_309251: bool,
        register_index: bool,
        wback: bool,
        D: bool,
        Rn: u8,
        Vd: u8,
        size: u8,
        index_align: u8,
        Rm: u8,
    }
    let fn_state = FunctionState {
        D,
        Rn,
        Vd,
        size,
        index_align,
        Rm,
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
        // N s_2_5: branch s_2_4 b11 b3
        if s_2_4 {
            return block_11(state, tracer, fn_state);
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
        // D s_3_1: read-var index_align:u8
        let s_3_1: u8 = fn_state.index_align;
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
        // C s_3_19: const #0u : u8
        let s_3_19: bool = false;
        // C s_3_20: cast zx s_3_19 -> bv
        let s_3_20: Bits = Bits::new(s_3_19 as u128, 1u16);
        // D s_3_21: cmp-ne s_3_18 s_3_20
        let s_3_21: bool = ((s_3_18) != (s_3_20));
        // N s_3_22: branch s_3_21 b10 b4
        if s_3_21 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1s : i
        let s_4_0: i128 = 1;
        // D s_4_1: read-var index_align:u8
        let s_4_1: u8 = fn_state.index_align;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 4u16);
        // C s_4_3: const #1s : i64
        let s_4_3: i64 = 1;
        // C s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // C s_4_5: const #2s : i
        let s_4_5: i128 = 2;
        // C s_4_6: add s_4_5 s_4_4
        let s_4_6: i128 = (s_4_5 + s_4_4);
        // D s_4_7: bit-extract s_4_2 s_4_0 s_4_6
        let s_4_7: Bits = (Bits::new(
            ((s_4_2) >> (s_4_0)).value(),
            u16::try_from(s_4_6).unwrap(),
        ));
        // D s_4_8: cast reint s_4_7 -> u8
        let s_4_8: u8 = (s_4_7.value() as u8);
        // D s_4_9: cast zx s_4_8 -> bv
        let s_4_9: Bits = Bits::new(s_4_8 as u128, 3u16);
        // D s_4_10: cast zx s_4_9 -> i
        let s_4_10: i128 = (s_4_9.value() as i128);
        // D s_4_11: cast reint s_4_10 -> i64
        let s_4_11: i64 = (s_4_10 as i64);
        // D s_4_12: write-var index <= s_4_11
        fn_state.index = s_4_11;
        // D s_4_13: read-var D:u8
        let s_4_13: bool = fn_state.D;
        // D s_4_14: cast zx s_4_13 -> bv
        let s_4_14: Bits = Bits::new(s_4_13 as u128, 1u16);
        // D s_4_15: read-var Vd:u8
        let s_4_15: u8 = fn_state.Vd;
        // D s_4_16: cast zx s_4_15 -> bv
        let s_4_16: Bits = Bits::new(s_4_15 as u128, 4u16);
        // D s_4_17: cast reint s_4_14 -> u128
        let s_4_17: u128 = (s_4_14.value() as u128);
        // D s_4_18: size-of s_4_14
        let s_4_18: u16 = s_4_14.length();
        // D s_4_19: cast reint s_4_16 -> u128
        let s_4_19: u128 = (s_4_16.value() as u128);
        // D s_4_20: size-of s_4_16
        let s_4_20: u16 = s_4_16.length();
        // D s_4_21: lsl s_4_17 s_4_20
        let s_4_21: u128 = s_4_17 << s_4_20;
        // D s_4_22: or s_4_21 s_4_19
        let s_4_22: u128 = ((s_4_21) | (s_4_19));
        // D s_4_23: add s_4_18 s_4_20
        let s_4_23: u16 = (s_4_18 + s_4_20);
        // D s_4_24: create-bits s_4_22 s_4_23
        let s_4_24: Bits = Bits::new(s_4_22, s_4_23);
        // D s_4_25: cast reint s_4_24 -> u8
        let s_4_25: u8 = (s_4_24.value() as u8);
        // D s_4_26: cast zx s_4_25 -> bv
        let s_4_26: Bits = Bits::new(s_4_25 as u128, 5u16);
        // D s_4_27: cast zx s_4_26 -> i
        let s_4_27: i128 = (s_4_26.value() as i128);
        // D s_4_28: cast reint s_4_27 -> i64
        let s_4_28: i64 = (s_4_27 as i64);
        // D s_4_29: write-var d <= s_4_28
        fn_state.d = s_4_28;
        // D s_4_30: read-var Rn:u8
        let s_4_30: u8 = fn_state.Rn;
        // D s_4_31: cast zx s_4_30 -> bv
        let s_4_31: Bits = Bits::new(s_4_30 as u128, 4u16);
        // D s_4_32: cast zx s_4_31 -> i
        let s_4_32: i128 = (s_4_31.value() as i128);
        // D s_4_33: cast reint s_4_32 -> i64
        let s_4_33: i64 = (s_4_32 as i64);
        // D s_4_34: write-var n <= s_4_33
        fn_state.n = s_4_33;
        // D s_4_35: read-var Rm:u8
        let s_4_35: u8 = fn_state.Rm;
        // D s_4_36: cast zx s_4_35 -> bv
        let s_4_36: Bits = Bits::new(s_4_35 as u128, 4u16);
        // D s_4_37: cast zx s_4_36 -> i
        let s_4_37: i128 = (s_4_36.value() as i128);
        // D s_4_38: cast reint s_4_37 -> i64
        let s_4_38: i64 = (s_4_37 as i64);
        // D s_4_39: write-var m <= s_4_38
        fn_state.m = s_4_38;
        // C s_4_40: const #15s : i
        let s_4_40: i128 = 15;
        // D s_4_41: read-var m:i64
        let s_4_41: i64 = fn_state.m;
        // D s_4_42: cast zx s_4_41 -> i
        let s_4_42: i128 = (i128::try_from(s_4_41).unwrap());
        // D s_4_43: call neq_int(s_4_42, s_4_40)
        let s_4_43: bool = neq_int(state, tracer, s_4_42, s_4_40);
        // D s_4_44: write-var wback <= s_4_43
        fn_state.wback = s_4_43;
        // C s_4_45: const #15s : i
        let s_4_45: i128 = 15;
        // D s_4_46: read-var m:i64
        let s_4_46: i64 = fn_state.m;
        // D s_4_47: cast zx s_4_46 -> i
        let s_4_47: i128 = (i128::try_from(s_4_46).unwrap());
        // D s_4_48: call neq_int(s_4_47, s_4_45)
        let s_4_48: bool = neq_int(state, tracer, s_4_47, s_4_45);
        // N s_4_49: branch s_4_48 b9 b5
        if s_4_48 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#309251 <= s_5_0
        fn_state.gs_309251 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#309251:u8
        let s_6_0: bool = fn_state.gs_309251;
        // D s_6_1: write-var register_index <= s_6_0
        fn_state.register_index = s_6_0;
        // C s_6_2: const #15s : i
        let s_6_2: i128 = 15;
        // D s_6_3: read-var n:i64
        let s_6_3: i64 = fn_state.n;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: cmp-eq s_6_4 s_6_2
        let s_6_5: bool = ((s_6_4) == (s_6_2));
        // N s_6_6: branch s_6_5 b8 b7
        if s_6_5 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1s : i64
        let s_7_0: i64 = 1;
        // D s_7_1: read-var d:i64
        let s_7_1: i64 = fn_state.d;
        // C s_7_2: const #1s : i64
        let s_7_2: i64 = 1;
        // D s_7_3: read-var index:i64
        let s_7_3: i64 = fn_state.index;
        // D s_7_4: read-var m:i64
        let s_7_4: i64 = fn_state.m;
        // D s_7_5: read-var n:i64
        let s_7_5: i64 = fn_state.n;
        // D s_7_6: read-var register_index:u8
        let s_7_6: bool = fn_state.register_index;
        // D s_7_7: read-var wback:u8
        let s_7_7: bool = fn_state.wback;
        // D s_7_8: call execute_aarch32_instrs_VLD1_1_Op_A_txt(s_7_0, s_7_1, s_7_2, s_7_3, s_7_4, s_7_5, s_7_6, s_7_7)
        let s_7_8: () = execute_aarch32_instrs_VLD1_1_Op_A_txt(
            state,
            tracer,
            s_7_0,
            s_7_1,
            s_7_2,
            s_7_3,
            s_7_4,
            s_7_5,
            s_7_6,
            s_7_7,
        );
        // N s_7_9: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: panic
        panic!("{:?}", ());
        // N s_8_1: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #13s : i
        let s_9_0: i128 = 13;
        // D s_9_1: read-var m:i64
        let s_9_1: i64 = fn_state.m;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: call neq_int(s_9_2, s_9_0)
        let s_9_3: bool = neq_int(state, tracer, s_9_2, s_9_0);
        // D s_9_4: write-var gs#309251 <= s_9_3
        fn_state.gs_309251 = s_9_3;
        // N s_9_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: panic
        panic!("{:?}", ());
        // N s_10_1: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: panic
        panic!("{:?}", ());
        // N s_11_1: return
        return;
    }
}
