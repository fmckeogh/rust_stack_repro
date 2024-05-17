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
use execute_aarch32_instrs_LDRB_i_OpT_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_LDRB_i_T3enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rn: u8,
    Rt: u8,
    P: bool,
    U: bool,
    W: bool,
    imm8: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_297053: bool,
        gs_297039: bool,
        t: i64,
        imm32: u32,
        gs_297052: bool,
        gs_297041: bool,
        gs_297040: bool,
        n: i64,
        gs_297038: bool,
        index: bool,
        add: bool,
        wback: bool,
        gs_297051: bool,
        gs_297042: bool,
        gs_297037: bool,
        Rn: u8,
        Rt: u8,
        P: bool,
        U: bool,
        W: bool,
        imm8: u8,
    }
    let fn_state = FunctionState {
        Rn,
        Rt,
        P,
        U,
        W,
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
        // D s_2_0: read-var Rt:u8
        let s_2_0: u8 = fn_state.Rt;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_2: const #15u : u8
        let s_2_2: u8 = 15;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b39 b3
        if s_2_4 {
            return block_39(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#297037 <= s_3_0
        fn_state.gs_297037 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#297037:u8
        let s_4_0: bool = fn_state.gs_297037;
        // N s_4_1: branch s_4_0 b38 b5
        if s_4_0 {
            return block_38(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#297038 <= s_5_0
        fn_state.gs_297038 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#297038:u8
        let s_6_0: bool = fn_state.gs_297038;
        // N s_6_1: branch s_6_0 b37 b7
        if s_6_0 {
            return block_37(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#297039 <= s_7_0
        fn_state.gs_297039 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#297039:u8
        let s_8_0: bool = fn_state.gs_297039;
        // N s_8_1: branch s_8_0 b36 b9
        if s_8_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var Rn:u8
        let s_9_0: u8 = fn_state.Rn;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 4u16);
        // C s_9_2: const #15u : u8
        let s_9_2: u8 = 15;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 4u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // N s_9_5: branch s_9_4 b35 b10
        if s_9_4 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var P:u8
        let s_10_0: bool = fn_state.P;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #1u : u8
        let s_10_2: bool = true;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // N s_10_5: branch s_10_4 b34 b11
        if s_10_4 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#297040 <= s_11_0
        fn_state.gs_297040 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#297040:u8
        let s_12_0: bool = fn_state.gs_297040;
        // N s_12_1: branch s_12_0 b33 b13
        if s_12_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#297041 <= s_13_0
        fn_state.gs_297041 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#297041:u8
        let s_14_0: bool = fn_state.gs_297041;
        // N s_14_1: branch s_14_0 b32 b15
        if s_14_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var P:u8
        let s_15_0: bool = fn_state.P;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 1u16);
        // C s_15_2: const #0u : u8
        let s_15_2: bool = false;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 1u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // N s_15_5: branch s_15_4 b31 b16
        if s_15_4 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#297042 <= s_16_0
        fn_state.gs_297042 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#297042:u8
        let s_17_0: bool = fn_state.gs_297042;
        // N s_17_1: branch s_17_0 b30 b18
        if s_17_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var Rt:u8
        let s_18_0: u8 = fn_state.Rt;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 4u16);
        // D s_18_2: cast zx s_18_1 -> i
        let s_18_2: i128 = (s_18_1.value() as i128);
        // D s_18_3: cast reint s_18_2 -> i64
        let s_18_3: i64 = (s_18_2 as i64);
        // D s_18_4: write-var t <= s_18_3
        fn_state.t = s_18_3;
        // D s_18_5: read-var Rn:u8
        let s_18_5: u8 = fn_state.Rn;
        // D s_18_6: cast zx s_18_5 -> bv
        let s_18_6: Bits = Bits::new(s_18_5 as u128, 4u16);
        // D s_18_7: cast zx s_18_6 -> i
        let s_18_7: i128 = (s_18_6.value() as i128);
        // D s_18_8: cast reint s_18_7 -> i64
        let s_18_8: i64 = (s_18_7 as i64);
        // D s_18_9: write-var n <= s_18_8
        fn_state.n = s_18_8;
        // C s_18_10: const #32s : i
        let s_18_10: i128 = 32;
        // D s_18_11: read-var imm8:u8
        let s_18_11: u8 = fn_state.imm8;
        // D s_18_12: cast zx s_18_11 -> bv
        let s_18_12: Bits = Bits::new(s_18_11 as u128, 8u16);
        // D s_18_13: bits-cast zx s_18_12 -> bv length s_18_10
        let s_18_13: Bits = s_18_12.zero_extend(s_18_10);
        // D s_18_14: cast reint s_18_13 -> u32
        let s_18_14: u32 = (s_18_13.value() as u32);
        // D s_18_15: write-var imm32 <= s_18_14
        fn_state.imm32 = s_18_14;
        // D s_18_16: read-var P:u8
        let s_18_16: bool = fn_state.P;
        // D s_18_17: cast zx s_18_16 -> bv
        let s_18_17: Bits = Bits::new(s_18_16 as u128, 1u16);
        // C s_18_18: const #1u : u8
        let s_18_18: bool = true;
        // C s_18_19: cast zx s_18_18 -> bv
        let s_18_19: Bits = Bits::new(s_18_18 as u128, 1u16);
        // D s_18_20: cmp-eq s_18_17 s_18_19
        let s_18_20: bool = ((s_18_17) == (s_18_19));
        // D s_18_21: write-var index <= s_18_20
        fn_state.index = s_18_20;
        // D s_18_22: read-var U:u8
        let s_18_22: bool = fn_state.U;
        // D s_18_23: cast zx s_18_22 -> bv
        let s_18_23: Bits = Bits::new(s_18_22 as u128, 1u16);
        // C s_18_24: const #1u : u8
        let s_18_24: bool = true;
        // C s_18_25: cast zx s_18_24 -> bv
        let s_18_25: Bits = Bits::new(s_18_24 as u128, 1u16);
        // D s_18_26: cmp-eq s_18_23 s_18_25
        let s_18_26: bool = ((s_18_23) == (s_18_25));
        // D s_18_27: write-var add <= s_18_26
        fn_state.add = s_18_26;
        // D s_18_28: read-var W:u8
        let s_18_28: bool = fn_state.W;
        // D s_18_29: cast zx s_18_28 -> bv
        let s_18_29: Bits = Bits::new(s_18_28 as u128, 1u16);
        // C s_18_30: const #1u : u8
        let s_18_30: bool = true;
        // C s_18_31: cast zx s_18_30 -> bv
        let s_18_31: Bits = Bits::new(s_18_30 as u128, 1u16);
        // D s_18_32: cmp-eq s_18_29 s_18_31
        let s_18_32: bool = ((s_18_29) == (s_18_31));
        // D s_18_33: write-var wback <= s_18_32
        fn_state.wback = s_18_32;
        // C s_18_34: const #15s : i
        let s_18_34: i128 = 15;
        // D s_18_35: read-var t:i64
        let s_18_35: i64 = fn_state.t;
        // D s_18_36: cast zx s_18_35 -> i
        let s_18_36: i128 = (i128::try_from(s_18_35).unwrap());
        // D s_18_37: cmp-eq s_18_36 s_18_34
        let s_18_37: bool = ((s_18_36) == (s_18_34));
        // N s_18_38: branch s_18_37 b29 b19
        if s_18_37 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#297051 <= s_19_0
        fn_state.gs_297051 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#297051:u8
        let s_20_0: bool = fn_state.gs_297051;
        // N s_20_1: branch s_20_0 b28 b21
        if s_20_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var wback:u8
        let s_21_0: bool = fn_state.wback;
        // N s_21_1: branch s_21_0 b27 b22
        if s_21_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#297052 <= s_22_0
        fn_state.gs_297052 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#297052:u8
        let s_23_0: bool = fn_state.gs_297052;
        // D s_23_1: write-var gs#297053 <= s_23_0
        fn_state.gs_297053 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#297053:u8
        let s_24_0: bool = fn_state.gs_297053;
        // N s_24_1: branch s_24_0 b26 b25
        if s_24_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var add:u8
        let s_25_0: bool = fn_state.add;
        // D s_25_1: read-var imm32:u32
        let s_25_1: u32 = fn_state.imm32;
        // D s_25_2: read-var index:u8
        let s_25_2: bool = fn_state.index;
        // D s_25_3: read-var n:i64
        let s_25_3: i64 = fn_state.n;
        // D s_25_4: read-var t:i64
        let s_25_4: i64 = fn_state.t;
        // D s_25_5: read-var wback:u8
        let s_25_5: bool = fn_state.wback;
        // D s_25_6: call execute_aarch32_instrs_LDRB_i_OpT_A_txt(s_25_0, s_25_1, s_25_2, s_25_3, s_25_4, s_25_5)
        let s_25_6: () = execute_aarch32_instrs_LDRB_i_OpT_A_txt(
            state,
            tracer,
            s_25_0,
            s_25_1,
            s_25_2,
            s_25_3,
            s_25_4,
            s_25_5,
        );
        // N s_25_7: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: panic
        panic!("{:?}", ());
        // N s_26_1: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var n:i64
        let s_27_0: i64 = fn_state.n;
        // D s_27_1: cast zx s_27_0 -> i
        let s_27_1: i128 = (i128::try_from(s_27_0).unwrap());
        // D s_27_2: read-var t:i64
        let s_27_2: i64 = fn_state.t;
        // D s_27_3: cast zx s_27_2 -> i
        let s_27_3: i128 = (i128::try_from(s_27_2).unwrap());
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#297052 <= s_27_4
        fn_state.gs_297052 = s_27_4;
        // N s_27_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#297053 <= s_28_0
        fn_state.gs_297053 = s_28_0;
        // N s_28_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var W:u8
        let s_29_0: bool = fn_state.W;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #1u : u8
        let s_29_2: bool = true;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#297051 <= s_29_4
        fn_state.gs_297051 = s_29_4;
        // N s_29_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: panic
        panic!("{:?}", ());
        // N s_30_1: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var W:u8
        let s_31_0: bool = fn_state.W;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 1u16);
        // C s_31_2: const #0u : u8
        let s_31_2: bool = false;
        // C s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // D s_31_5: write-var gs#297042 <= s_31_4
        fn_state.gs_297042 = s_31_4;
        // N s_31_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_32_0: panic
        panic!("{:?}", ());
        // N s_32_1: return
        return;
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var W:u8
        let s_33_0: bool = fn_state.W;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // C s_33_2: const #0u : u8
        let s_33_2: bool = false;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: write-var gs#297041 <= s_33_4
        fn_state.gs_297041 = s_33_4;
        // N s_33_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var U:u8
        let s_34_0: bool = fn_state.U;
        // D s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 1u16);
        // C s_34_2: const #1u : u8
        let s_34_2: bool = true;
        // C s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 1u16);
        // D s_34_4: cmp-eq s_34_1 s_34_3
        let s_34_4: bool = ((s_34_1) == (s_34_3));
        // D s_34_5: write-var gs#297040 <= s_34_4
        fn_state.gs_297040 = s_34_4;
        // N s_34_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: panic
        panic!("{:?}", ());
        // N s_35_1: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_36_0: panic
        panic!("{:?}", ());
        // N s_36_1: return
        return;
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var W:u8
        let s_37_0: bool = fn_state.W;
        // D s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 1u16);
        // C s_37_2: const #0u : u8
        let s_37_2: bool = false;
        // C s_37_3: cast zx s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 1u16);
        // D s_37_4: cmp-eq s_37_1 s_37_3
        let s_37_4: bool = ((s_37_1) == (s_37_3));
        // D s_37_5: write-var gs#297039 <= s_37_4
        fn_state.gs_297039 = s_37_4;
        // N s_37_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var U:u8
        let s_38_0: bool = fn_state.U;
        // D s_38_1: cast zx s_38_0 -> bv
        let s_38_1: Bits = Bits::new(s_38_0 as u128, 1u16);
        // C s_38_2: const #0u : u8
        let s_38_2: bool = false;
        // C s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 1u16);
        // D s_38_4: cmp-eq s_38_1 s_38_3
        let s_38_4: bool = ((s_38_1) == (s_38_3));
        // D s_38_5: write-var gs#297038 <= s_38_4
        fn_state.gs_297038 = s_38_4;
        // N s_38_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var P:u8
        let s_39_0: bool = fn_state.P;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 1u16);
        // C s_39_2: const #1u : u8
        let s_39_2: bool = true;
        // C s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // D s_39_4: cmp-eq s_39_1 s_39_3
        let s_39_4: bool = ((s_39_1) == (s_39_3));
        // D s_39_5: write-var gs#297037 <= s_39_4
        fn_state.gs_297037 = s_39_4;
        // N s_39_6: jump b4
        return block_4(state, tracer, fn_state);
    }
}
