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
use execute_aarch32_instrs_STRB_i_OpA_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_STRB_i_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    P: bool,
    U: bool,
    W: bool,
    Rn: u8,
    Rt: u8,
    imm12: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        t: i64,
        imm32: u32,
        gs_302314: bool,
        n: i64,
        index: bool,
        gs_302315: bool,
        add: bool,
        wback: bool,
        gs_302305: bool,
        gs_302311: bool,
        cond: u8,
        P: bool,
        U: bool,
        W: bool,
        Rn: u8,
        Rt: u8,
        imm12: u16,
    }
    let fn_state = FunctionState {
        cond,
        P,
        U,
        W,
        Rn,
        Rt,
        imm12,
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
        // D s_2_0: read-var cond:u8
        let s_2_0: u8 = fn_state.cond;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_2: const #15u : u8
        let s_2_2: u8 = 15;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-ne s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) != (s_2_3));
        // N s_2_5: assert s_2_4
        let s_2_5: () = assert!(s_2_4);
        // D s_2_6: read-var P:u8
        let s_2_6: bool = fn_state.P;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 1u16);
        // C s_2_8: const #0u : u8
        let s_2_8: bool = false;
        // C s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 1u16);
        // D s_2_10: cmp-eq s_2_7 s_2_9
        let s_2_10: bool = ((s_2_7) == (s_2_9));
        // N s_2_11: branch s_2_10 b20 b3
        if s_2_10 {
            return block_20(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#302305 <= s_3_0
        fn_state.gs_302305 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#302305:u8
        let s_4_0: bool = fn_state.gs_302305;
        // N s_4_1: branch s_4_0 b19 b5
        if s_4_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var Rt:u8
        let s_5_0: u8 = fn_state.Rt;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 4u16);
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (s_5_1.value() as i128);
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // D s_5_4: write-var t <= s_5_3
        fn_state.t = s_5_3;
        // D s_5_5: read-var Rn:u8
        let s_5_5: u8 = fn_state.Rn;
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 4u16);
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (s_5_6.value() as i128);
        // D s_5_8: cast reint s_5_7 -> i64
        let s_5_8: i64 = (s_5_7 as i64);
        // D s_5_9: write-var n <= s_5_8
        fn_state.n = s_5_8;
        // C s_5_10: const #32s : i
        let s_5_10: i128 = 32;
        // D s_5_11: read-var imm12:u12
        let s_5_11: u16 = fn_state.imm12;
        // D s_5_12: cast zx s_5_11 -> bv
        let s_5_12: Bits = Bits::new(s_5_11 as u128, 12u16);
        // D s_5_13: bits-cast zx s_5_12 -> bv length s_5_10
        let s_5_13: Bits = s_5_12.zero_extend(s_5_10);
        // D s_5_14: cast reint s_5_13 -> u32
        let s_5_14: u32 = (s_5_13.value() as u32);
        // D s_5_15: write-var imm32 <= s_5_14
        fn_state.imm32 = s_5_14;
        // D s_5_16: read-var P:u8
        let s_5_16: bool = fn_state.P;
        // D s_5_17: cast zx s_5_16 -> bv
        let s_5_17: Bits = Bits::new(s_5_16 as u128, 1u16);
        // C s_5_18: const #1u : u8
        let s_5_18: bool = true;
        // C s_5_19: cast zx s_5_18 -> bv
        let s_5_19: Bits = Bits::new(s_5_18 as u128, 1u16);
        // D s_5_20: cmp-eq s_5_17 s_5_19
        let s_5_20: bool = ((s_5_17) == (s_5_19));
        // D s_5_21: write-var index <= s_5_20
        fn_state.index = s_5_20;
        // D s_5_22: read-var U:u8
        let s_5_22: bool = fn_state.U;
        // D s_5_23: cast zx s_5_22 -> bv
        let s_5_23: Bits = Bits::new(s_5_22 as u128, 1u16);
        // C s_5_24: const #1u : u8
        let s_5_24: bool = true;
        // C s_5_25: cast zx s_5_24 -> bv
        let s_5_25: Bits = Bits::new(s_5_24 as u128, 1u16);
        // D s_5_26: cmp-eq s_5_23 s_5_25
        let s_5_26: bool = ((s_5_23) == (s_5_25));
        // D s_5_27: write-var add <= s_5_26
        fn_state.add = s_5_26;
        // D s_5_28: read-var P:u8
        let s_5_28: bool = fn_state.P;
        // D s_5_29: cast zx s_5_28 -> bv
        let s_5_29: Bits = Bits::new(s_5_28 as u128, 1u16);
        // C s_5_30: const #0u : u8
        let s_5_30: bool = false;
        // C s_5_31: cast zx s_5_30 -> bv
        let s_5_31: Bits = Bits::new(s_5_30 as u128, 1u16);
        // D s_5_32: cmp-eq s_5_29 s_5_31
        let s_5_32: bool = ((s_5_29) == (s_5_31));
        // N s_5_33: branch s_5_32 b18 b6
        if s_5_32 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var W:u8
        let s_6_0: bool = fn_state.W;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // C s_6_2: const #1u : u8
        let s_6_2: bool = true;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // D s_6_5: write-var gs#302311 <= s_6_4
        fn_state.gs_302311 = s_6_4;
        // N s_6_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#302311:u8
        let s_7_0: bool = fn_state.gs_302311;
        // D s_7_1: write-var wback <= s_7_0
        fn_state.wback = s_7_0;
        // C s_7_2: const #15s : i
        let s_7_2: i128 = 15;
        // D s_7_3: read-var t:i64
        let s_7_3: i64 = fn_state.t;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: cmp-eq s_7_4 s_7_2
        let s_7_5: bool = ((s_7_4) == (s_7_2));
        // N s_7_6: branch s_7_5 b17 b8
        if s_7_5 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var wback:u8
        let s_8_0: bool = fn_state.wback;
        // N s_8_1: branch s_8_0 b13 b9
        if s_8_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#302315 <= s_9_0
        fn_state.gs_302315 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#302315:u8
        let s_10_0: bool = fn_state.gs_302315;
        // N s_10_1: branch s_10_0 b12 b11
        if s_10_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var add:u8
        let s_11_0: bool = fn_state.add;
        // D s_11_1: read-var imm32:u32
        let s_11_1: u32 = fn_state.imm32;
        // D s_11_2: read-var index:u8
        let s_11_2: bool = fn_state.index;
        // D s_11_3: read-var n:i64
        let s_11_3: i64 = fn_state.n;
        // D s_11_4: read-var t:i64
        let s_11_4: i64 = fn_state.t;
        // D s_11_5: read-var wback:u8
        let s_11_5: bool = fn_state.wback;
        // D s_11_6: call execute_aarch32_instrs_STRB_i_OpA_A_txt(s_11_0, s_11_1, s_11_2, s_11_3, s_11_4, s_11_5)
        let s_11_6: () = execute_aarch32_instrs_STRB_i_OpA_A_txt(
            state,
            tracer,
            s_11_0,
            s_11_1,
            s_11_2,
            s_11_3,
            s_11_4,
            s_11_5,
        );
        // N s_11_7: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: panic
        panic!("{:?}", ());
        // N s_12_1: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #15s : i
        let s_13_0: i128 = 15;
        // D s_13_1: read-var n:i64
        let s_13_1: i64 = fn_state.n;
        // D s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (i128::try_from(s_13_1).unwrap());
        // D s_13_3: cmp-eq s_13_2 s_13_0
        let s_13_3: bool = ((s_13_2) == (s_13_0));
        // N s_13_4: branch s_13_3 b16 b14
        if s_13_3 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var n:i64
        let s_14_0: i64 = fn_state.n;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: read-var t:i64
        let s_14_2: i64 = fn_state.t;
        // D s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // D s_14_5: write-var gs#302314 <= s_14_4
        fn_state.gs_302314 = s_14_4;
        // N s_14_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#302314:u8
        let s_15_0: bool = fn_state.gs_302314;
        // D s_15_1: write-var gs#302315 <= s_15_0
        fn_state.gs_302315 = s_15_0;
        // N s_15_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#302314 <= s_16_0
        fn_state.gs_302314 = s_16_0;
        // N s_16_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: panic
        panic!("{:?}", ());
        // N s_17_1: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#302311 <= s_18_0
        fn_state.gs_302311 = s_18_0;
        // N s_18_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: panic
        panic!("{:?}", ());
        // N s_19_1: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var W:u8
        let s_20_0: bool = fn_state.W;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 1u16);
        // C s_20_2: const #1u : u8
        let s_20_2: bool = true;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 1u16);
        // D s_20_4: cmp-eq s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) == (s_20_3));
        // D s_20_5: write-var gs#302305 <= s_20_4
        fn_state.gs_302305 = s_20_4;
        // N s_20_6: jump b4
        return block_4(state, tracer, fn_state);
    }
}
