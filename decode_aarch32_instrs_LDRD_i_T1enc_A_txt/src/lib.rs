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
use place_slice::*;
use execute_aarch32_instrs_LDRD_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_LDRD_i_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    P: bool,
    U: bool,
    W: bool,
    Rn: u8,
    Rt: u8,
    Rt2: u8,
    imm8: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        t: i64,
        imm32: u32,
        t2: i64,
        gs_297242: bool,
        gs_297256: bool,
        n: i64,
        index: bool,
        gs_297253: bool,
        add: bool,
        gs_297252: bool,
        gs_297257: bool,
        wback: bool,
        P: bool,
        U: bool,
        W: bool,
        Rn: u8,
        Rt: u8,
        Rt2: u8,
        imm8: u8,
    }
    let fn_state = FunctionState {
        P,
        U,
        W,
        Rn,
        Rt,
        Rt2,
        imm8,
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
        // D s_2_0: read-var P:u8
        let s_2_0: bool = fn_state.P;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #0u : u8
        let s_2_2: bool = false;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b25 b3
        if s_2_4 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#297242 <= s_3_0
        fn_state.gs_297242 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#297242:u8
        let s_4_0: bool = fn_state.gs_297242;
        // N s_4_1: branch s_4_0 b24 b5
        if s_4_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var Rn:u8
        let s_5_0: u8 = fn_state.Rn;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 4u16);
        // C s_5_2: const #15u : u8
        let s_5_2: u8 = 15;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 4u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b23 b6
        if s_5_4 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var Rt:u8
        let s_6_0: u8 = fn_state.Rt;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 4u16);
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (s_6_1.value() as i128);
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // D s_6_4: write-var t <= s_6_3
        fn_state.t = s_6_3;
        // D s_6_5: read-var Rt2:u8
        let s_6_5: u8 = fn_state.Rt2;
        // D s_6_6: cast zx s_6_5 -> bv
        let s_6_6: Bits = Bits::new(s_6_5 as u128, 4u16);
        // D s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (s_6_6.value() as i128);
        // D s_6_8: cast reint s_6_7 -> i64
        let s_6_8: i64 = (s_6_7 as i64);
        // D s_6_9: write-var t2 <= s_6_8
        fn_state.t2 = s_6_8;
        // D s_6_10: read-var Rn:u8
        let s_6_10: u8 = fn_state.Rn;
        // D s_6_11: cast zx s_6_10 -> bv
        let s_6_11: Bits = Bits::new(s_6_10 as u128, 4u16);
        // D s_6_12: cast zx s_6_11 -> i
        let s_6_12: i128 = (s_6_11.value() as i128);
        // D s_6_13: cast reint s_6_12 -> i64
        let s_6_13: i64 = (s_6_12 as i64);
        // D s_6_14: write-var n <= s_6_13
        fn_state.n = s_6_13;
        // C s_6_15: const #32s : i
        let s_6_15: i128 = 32;
        // C s_6_16: const #0s : i
        let s_6_16: i128 = 0;
        // C s_6_17: const #8s : i
        let s_6_17: i128 = 8;
        // C s_6_18: const #2s : i
        let s_6_18: i128 = 2;
        // D s_6_19: read-var imm8:u8
        let s_6_19: u8 = fn_state.imm8;
        // D s_6_20: cast zx s_6_19 -> bv
        let s_6_20: Bits = Bits::new(s_6_19 as u128, 8u16);
        // D s_6_21: call place_slice(s_6_15, s_6_20, s_6_16, s_6_17, s_6_18)
        let s_6_21: Bits = place_slice(
            state,
            tracer,
            s_6_15,
            s_6_20,
            s_6_16,
            s_6_17,
            s_6_18,
        );
        // D s_6_22: cast reint s_6_21 -> u32
        let s_6_22: u32 = (s_6_21.value() as u32);
        // D s_6_23: write-var imm32 <= s_6_22
        fn_state.imm32 = s_6_22;
        // D s_6_24: read-var P:u8
        let s_6_24: bool = fn_state.P;
        // D s_6_25: cast zx s_6_24 -> bv
        let s_6_25: Bits = Bits::new(s_6_24 as u128, 1u16);
        // C s_6_26: const #1u : u8
        let s_6_26: bool = true;
        // C s_6_27: cast zx s_6_26 -> bv
        let s_6_27: Bits = Bits::new(s_6_26 as u128, 1u16);
        // D s_6_28: cmp-eq s_6_25 s_6_27
        let s_6_28: bool = ((s_6_25) == (s_6_27));
        // D s_6_29: write-var index <= s_6_28
        fn_state.index = s_6_28;
        // D s_6_30: read-var U:u8
        let s_6_30: bool = fn_state.U;
        // D s_6_31: cast zx s_6_30 -> bv
        let s_6_31: Bits = Bits::new(s_6_30 as u128, 1u16);
        // C s_6_32: const #1u : u8
        let s_6_32: bool = true;
        // C s_6_33: cast zx s_6_32 -> bv
        let s_6_33: Bits = Bits::new(s_6_32 as u128, 1u16);
        // D s_6_34: cmp-eq s_6_31 s_6_33
        let s_6_34: bool = ((s_6_31) == (s_6_33));
        // D s_6_35: write-var add <= s_6_34
        fn_state.add = s_6_34;
        // D s_6_36: read-var W:u8
        let s_6_36: bool = fn_state.W;
        // D s_6_37: cast zx s_6_36 -> bv
        let s_6_37: Bits = Bits::new(s_6_36 as u128, 1u16);
        // C s_6_38: const #1u : u8
        let s_6_38: bool = true;
        // C s_6_39: cast zx s_6_38 -> bv
        let s_6_39: Bits = Bits::new(s_6_38 as u128, 1u16);
        // D s_6_40: cmp-eq s_6_37 s_6_39
        let s_6_40: bool = ((s_6_37) == (s_6_39));
        // D s_6_41: write-var wback <= s_6_40
        fn_state.wback = s_6_40;
        // D s_6_42: read-var wback:u8
        let s_6_42: bool = fn_state.wback;
        // N s_6_43: branch s_6_42 b19 b7
        if s_6_42 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#297253 <= s_7_0
        fn_state.gs_297253 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#297253:u8
        let s_8_0: bool = fn_state.gs_297253;
        // N s_8_1: branch s_8_0 b18 b9
        if s_8_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #15s : i
        let s_9_0: i128 = 15;
        // D s_9_1: read-var t:i64
        let s_9_1: i64 = fn_state.t;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: cmp-eq s_9_2 s_9_0
        let s_9_3: bool = ((s_9_2) == (s_9_0));
        // N s_9_4: branch s_9_3 b17 b10
        if s_9_3 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #15s : i
        let s_10_0: i128 = 15;
        // D s_10_1: read-var t2:i64
        let s_10_1: i64 = fn_state.t2;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: cmp-eq s_10_2 s_10_0
        let s_10_3: bool = ((s_10_2) == (s_10_0));
        // D s_10_4: write-var gs#297256 <= s_10_3
        fn_state.gs_297256 = s_10_3;
        // N s_10_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#297256:u8
        let s_11_0: bool = fn_state.gs_297256;
        // N s_11_1: branch s_11_0 b16 b12
        if s_11_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var t:i64
        let s_12_0: i64 = fn_state.t;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: read-var t2:i64
        let s_12_2: i64 = fn_state.t2;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: write-var gs#297257 <= s_12_4
        fn_state.gs_297257 = s_12_4;
        // N s_12_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#297257:u8
        let s_13_0: bool = fn_state.gs_297257;
        // N s_13_1: branch s_13_0 b15 b14
        if s_13_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var add:u8
        let s_14_0: bool = fn_state.add;
        // D s_14_1: read-var imm32:u32
        let s_14_1: u32 = fn_state.imm32;
        // D s_14_2: read-var index:u8
        let s_14_2: bool = fn_state.index;
        // D s_14_3: read-var n:i64
        let s_14_3: i64 = fn_state.n;
        // D s_14_4: read-var t:i64
        let s_14_4: i64 = fn_state.t;
        // D s_14_5: read-var t2:i64
        let s_14_5: i64 = fn_state.t2;
        // D s_14_6: read-var wback:u8
        let s_14_6: bool = fn_state.wback;
        // D s_14_7: call execute_aarch32_instrs_LDRD_i_Op_A_txt(s_14_0, s_14_1, s_14_2, s_14_3, s_14_4, s_14_5, s_14_6)
        let s_14_7: () = execute_aarch32_instrs_LDRD_i_Op_A_txt(
            state,
            tracer,
            s_14_0,
            s_14_1,
            s_14_2,
            s_14_3,
            s_14_4,
            s_14_5,
            s_14_6,
        );
        // N s_14_8: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#297257 <= s_16_0
        fn_state.gs_297257 = s_16_0;
        // N s_16_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#297256 <= s_17_0
        fn_state.gs_297256 = s_17_0;
        // N s_17_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: panic
        panic!("{:?}", ());
        // N s_18_1: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var n:i64
        let s_19_0: i64 = fn_state.n;
        // D s_19_1: cast zx s_19_0 -> i
        let s_19_1: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_2: read-var t:i64
        let s_19_2: i64 = fn_state.t;
        // D s_19_3: cast zx s_19_2 -> i
        let s_19_3: i128 = (i128::try_from(s_19_2).unwrap());
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // N s_19_5: branch s_19_4 b22 b20
        if s_19_4 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var n:i64
        let s_20_0: i64 = fn_state.n;
        // D s_20_1: cast zx s_20_0 -> i
        let s_20_1: i128 = (i128::try_from(s_20_0).unwrap());
        // D s_20_2: read-var t2:i64
        let s_20_2: i64 = fn_state.t2;
        // D s_20_3: cast zx s_20_2 -> i
        let s_20_3: i128 = (i128::try_from(s_20_2).unwrap());
        // D s_20_4: cmp-eq s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) == (s_20_3));
        // D s_20_5: write-var gs#297252 <= s_20_4
        fn_state.gs_297252 = s_20_4;
        // N s_20_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#297252:u8
        let s_21_0: bool = fn_state.gs_297252;
        // D s_21_1: write-var gs#297253 <= s_21_0
        fn_state.gs_297253 = s_21_0;
        // N s_21_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#297252 <= s_22_0
        fn_state.gs_297252 = s_22_0;
        // N s_22_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: panic
        panic!("{:?}", ());
        // N s_23_1: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: panic
        panic!("{:?}", ());
        // N s_24_1: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var W:u8
        let s_25_0: bool = fn_state.W;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #0u : u8
        let s_25_2: bool = false;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#297242 <= s_25_4
        fn_state.gs_297242 = s_25_4;
        // N s_25_6: jump b4
        return block_4(state, tracer, fn_state);
    }
}
