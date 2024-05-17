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
use execute_aarch32_instrs_LDRH_i_OpA_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_LDRH_i_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    P: bool,
    U: bool,
    W: bool,
    Rn: u8,
    Rt: u8,
    imm4H: u8,
    imm4L: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        t: i64,
        imm32: u32,
        n: i64,
        index: bool,
        gs_297487: bool,
        add: bool,
        wback: bool,
        gs_297489: bool,
        gs_297490: bool,
        gs_297480: bool,
        cond: u8,
        P: bool,
        U: bool,
        W: bool,
        Rn: u8,
        Rt: u8,
        imm4H: u8,
        imm4L: u8,
    }
    let fn_state = FunctionState {
        cond,
        P,
        U,
        W,
        Rn,
        Rt,
        imm4H,
        imm4L,
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
        // D s_2_6: read-var Rn:u8
        let s_2_6: u8 = fn_state.Rn;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 4u16);
        // C s_2_8: const #15u : u8
        let s_2_8: u8 = 15;
        // C s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 4u16);
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
        // D s_3_0: read-var P:u8
        let s_3_0: bool = fn_state.P;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // C s_3_2: const #0u : u8
        let s_3_2: bool = false;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b19 b4
        if s_3_4 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#297480 <= s_4_0
        fn_state.gs_297480 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#297480:u8
        let s_5_0: bool = fn_state.gs_297480;
        // N s_5_1: branch s_5_0 b18 b6
        if s_5_0 {
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
        // D s_6_5: read-var Rn:u8
        let s_6_5: u8 = fn_state.Rn;
        // D s_6_6: cast zx s_6_5 -> bv
        let s_6_6: Bits = Bits::new(s_6_5 as u128, 4u16);
        // D s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (s_6_6.value() as i128);
        // D s_6_8: cast reint s_6_7 -> i64
        let s_6_8: i64 = (s_6_7 as i64);
        // D s_6_9: write-var n <= s_6_8
        fn_state.n = s_6_8;
        // D s_6_10: read-var imm4H:u8
        let s_6_10: u8 = fn_state.imm4H;
        // D s_6_11: cast zx s_6_10 -> bv
        let s_6_11: Bits = Bits::new(s_6_10 as u128, 4u16);
        // D s_6_12: read-var imm4L:u8
        let s_6_12: u8 = fn_state.imm4L;
        // D s_6_13: cast zx s_6_12 -> bv
        let s_6_13: Bits = Bits::new(s_6_12 as u128, 4u16);
        // D s_6_14: cast reint s_6_11 -> u128
        let s_6_14: u128 = (s_6_11.value() as u128);
        // D s_6_15: size-of s_6_11
        let s_6_15: u16 = s_6_11.length();
        // D s_6_16: cast reint s_6_13 -> u128
        let s_6_16: u128 = (s_6_13.value() as u128);
        // D s_6_17: size-of s_6_13
        let s_6_17: u16 = s_6_13.length();
        // D s_6_18: lsl s_6_14 s_6_17
        let s_6_18: u128 = s_6_14 << s_6_17;
        // D s_6_19: or s_6_18 s_6_16
        let s_6_19: u128 = ((s_6_18) | (s_6_16));
        // D s_6_20: add s_6_15 s_6_17
        let s_6_20: u16 = (s_6_15 + s_6_17);
        // D s_6_21: create-bits s_6_19 s_6_20
        let s_6_21: Bits = Bits::new(s_6_19, s_6_20);
        // D s_6_22: cast reint s_6_21 -> u8
        let s_6_22: u8 = (s_6_21.value() as u8);
        // C s_6_23: const #32s : i
        let s_6_23: i128 = 32;
        // D s_6_24: cast zx s_6_22 -> bv
        let s_6_24: Bits = Bits::new(s_6_22 as u128, 8u16);
        // D s_6_25: bits-cast zx s_6_24 -> bv length s_6_23
        let s_6_25: Bits = s_6_24.zero_extend(s_6_23);
        // D s_6_26: cast reint s_6_25 -> u32
        let s_6_26: u32 = (s_6_25.value() as u32);
        // D s_6_27: write-var imm32 <= s_6_26
        fn_state.imm32 = s_6_26;
        // D s_6_28: read-var P:u8
        let s_6_28: bool = fn_state.P;
        // D s_6_29: cast zx s_6_28 -> bv
        let s_6_29: Bits = Bits::new(s_6_28 as u128, 1u16);
        // C s_6_30: const #1u : u8
        let s_6_30: bool = true;
        // C s_6_31: cast zx s_6_30 -> bv
        let s_6_31: Bits = Bits::new(s_6_30 as u128, 1u16);
        // D s_6_32: cmp-eq s_6_29 s_6_31
        let s_6_32: bool = ((s_6_29) == (s_6_31));
        // D s_6_33: write-var index <= s_6_32
        fn_state.index = s_6_32;
        // D s_6_34: read-var U:u8
        let s_6_34: bool = fn_state.U;
        // D s_6_35: cast zx s_6_34 -> bv
        let s_6_35: Bits = Bits::new(s_6_34 as u128, 1u16);
        // C s_6_36: const #1u : u8
        let s_6_36: bool = true;
        // C s_6_37: cast zx s_6_36 -> bv
        let s_6_37: Bits = Bits::new(s_6_36 as u128, 1u16);
        // D s_6_38: cmp-eq s_6_35 s_6_37
        let s_6_38: bool = ((s_6_35) == (s_6_37));
        // D s_6_39: write-var add <= s_6_38
        fn_state.add = s_6_38;
        // D s_6_40: read-var P:u8
        let s_6_40: bool = fn_state.P;
        // D s_6_41: cast zx s_6_40 -> bv
        let s_6_41: Bits = Bits::new(s_6_40 as u128, 1u16);
        // C s_6_42: const #0u : u8
        let s_6_42: bool = false;
        // C s_6_43: cast zx s_6_42 -> bv
        let s_6_43: Bits = Bits::new(s_6_42 as u128, 1u16);
        // D s_6_44: cmp-eq s_6_41 s_6_43
        let s_6_44: bool = ((s_6_41) == (s_6_43));
        // N s_6_45: branch s_6_44 b17 b7
        if s_6_44 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var W:u8
        let s_7_0: bool = fn_state.W;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 1u16);
        // C s_7_2: const #1u : u8
        let s_7_2: bool = true;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // D s_7_5: write-var gs#297487 <= s_7_4
        fn_state.gs_297487 = s_7_4;
        // N s_7_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#297487:u8
        let s_8_0: bool = fn_state.gs_297487;
        // D s_8_1: write-var wback <= s_8_0
        fn_state.wback = s_8_0;
        // C s_8_2: const #15s : i
        let s_8_2: i128 = 15;
        // D s_8_3: read-var t:i64
        let s_8_3: i64 = fn_state.t;
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_5: cmp-eq s_8_4 s_8_2
        let s_8_5: bool = ((s_8_4) == (s_8_2));
        // N s_8_6: branch s_8_5 b16 b9
        if s_8_5 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var wback:u8
        let s_9_0: bool = fn_state.wback;
        // N s_9_1: branch s_9_0 b15 b10
        if s_9_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#297489 <= s_10_0
        fn_state.gs_297489 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#297489:u8
        let s_11_0: bool = fn_state.gs_297489;
        // D s_11_1: write-var gs#297490 <= s_11_0
        fn_state.gs_297490 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#297490:u8
        let s_12_0: bool = fn_state.gs_297490;
        // N s_12_1: branch s_12_0 b14 b13
        if s_12_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var add:u8
        let s_13_0: bool = fn_state.add;
        // D s_13_1: read-var imm32:u32
        let s_13_1: u32 = fn_state.imm32;
        // D s_13_2: read-var index:u8
        let s_13_2: bool = fn_state.index;
        // D s_13_3: read-var n:i64
        let s_13_3: i64 = fn_state.n;
        // D s_13_4: read-var t:i64
        let s_13_4: i64 = fn_state.t;
        // D s_13_5: read-var wback:u8
        let s_13_5: bool = fn_state.wback;
        // D s_13_6: call execute_aarch32_instrs_LDRH_i_OpA_A_txt(s_13_0, s_13_1, s_13_2, s_13_3, s_13_4, s_13_5)
        let s_13_6: () = execute_aarch32_instrs_LDRH_i_OpA_A_txt(
            state,
            tracer,
            s_13_0,
            s_13_1,
            s_13_2,
            s_13_3,
            s_13_4,
            s_13_5,
        );
        // N s_13_7: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: panic
        panic!("{:?}", ());
        // N s_14_1: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var n:i64
        let s_15_0: i64 = fn_state.n;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: read-var t:i64
        let s_15_2: i64 = fn_state.t;
        // D s_15_3: cast zx s_15_2 -> i
        let s_15_3: i128 = (i128::try_from(s_15_2).unwrap());
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // D s_15_5: write-var gs#297489 <= s_15_4
        fn_state.gs_297489 = s_15_4;
        // N s_15_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#297490 <= s_16_0
        fn_state.gs_297490 = s_16_0;
        // N s_16_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#297487 <= s_17_0
        fn_state.gs_297487 = s_17_0;
        // N s_17_2: jump b8
        return block_8(state, tracer, fn_state);
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
        // D s_19_0: read-var W:u8
        let s_19_0: bool = fn_state.W;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 1u16);
        // C s_19_2: const #1u : u8
        let s_19_2: bool = true;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // D s_19_5: write-var gs#297480 <= s_19_4
        fn_state.gs_297480 = s_19_4;
        // N s_19_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: panic
        panic!("{:?}", ());
        // N s_20_1: return
        return;
    }
}
