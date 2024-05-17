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
use execute_aarch32_instrs_LDR_i_OpT_A_txt::*;
use LastInITBlock::*;
use ConditionPassed::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_LDR_i_T4enc_A_txt<T: Tracer>(
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
        t: i64,
        imm32: u32,
        gs_297740: bool,
        gs_297741: bool,
        gs_297752: bool,
        gs_297742: bool,
        gs_297749: bool,
        n: i64,
        index: bool,
        add: bool,
        wback: bool,
        gs_297753: bool,
        gs_297751: bool,
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
        // D s_2_0: read-var Rn:u8
        let s_2_0: u8 = fn_state.Rn;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_2: const #15u : u8
        let s_2_2: u8 = 15;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b31 b3
        if s_2_4 {
            return block_31(state, tracer, fn_state);
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
        // C s_3_2: const #1u : u8
        let s_3_2: bool = true;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b30 b4
        if s_3_4 {
            return block_30(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#297740 <= s_4_0
        fn_state.gs_297740 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#297740:u8
        let s_5_0: bool = fn_state.gs_297740;
        // N s_5_1: branch s_5_0 b29 b6
        if s_5_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#297741 <= s_6_0
        fn_state.gs_297741 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#297741:u8
        let s_7_0: bool = fn_state.gs_297741;
        // N s_7_1: branch s_7_0 b28 b8
        if s_7_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var P:u8
        let s_8_0: bool = fn_state.P;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 1u16);
        // C s_8_2: const #0u : u8
        let s_8_2: bool = false;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // N s_8_5: branch s_8_4 b27 b9
        if s_8_4 {
            return block_27(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#297742 <= s_9_0
        fn_state.gs_297742 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#297742:u8
        let s_10_0: bool = fn_state.gs_297742;
        // N s_10_1: branch s_10_0 b26 b11
        if s_10_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var Rt:u8
        let s_11_0: u8 = fn_state.Rt;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 4u16);
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (s_11_1.value() as i128);
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // D s_11_4: write-var t <= s_11_3
        fn_state.t = s_11_3;
        // D s_11_5: read-var Rn:u8
        let s_11_5: u8 = fn_state.Rn;
        // D s_11_6: cast zx s_11_5 -> bv
        let s_11_6: Bits = Bits::new(s_11_5 as u128, 4u16);
        // D s_11_7: cast zx s_11_6 -> i
        let s_11_7: i128 = (s_11_6.value() as i128);
        // D s_11_8: cast reint s_11_7 -> i64
        let s_11_8: i64 = (s_11_7 as i64);
        // D s_11_9: write-var n <= s_11_8
        fn_state.n = s_11_8;
        // C s_11_10: const #32s : i
        let s_11_10: i128 = 32;
        // D s_11_11: read-var imm8:u8
        let s_11_11: u8 = fn_state.imm8;
        // D s_11_12: cast zx s_11_11 -> bv
        let s_11_12: Bits = Bits::new(s_11_11 as u128, 8u16);
        // D s_11_13: bits-cast zx s_11_12 -> bv length s_11_10
        let s_11_13: Bits = s_11_12.zero_extend(s_11_10);
        // D s_11_14: cast reint s_11_13 -> u32
        let s_11_14: u32 = (s_11_13.value() as u32);
        // D s_11_15: write-var imm32 <= s_11_14
        fn_state.imm32 = s_11_14;
        // D s_11_16: read-var P:u8
        let s_11_16: bool = fn_state.P;
        // D s_11_17: cast zx s_11_16 -> bv
        let s_11_17: Bits = Bits::new(s_11_16 as u128, 1u16);
        // C s_11_18: const #1u : u8
        let s_11_18: bool = true;
        // C s_11_19: cast zx s_11_18 -> bv
        let s_11_19: Bits = Bits::new(s_11_18 as u128, 1u16);
        // D s_11_20: cmp-eq s_11_17 s_11_19
        let s_11_20: bool = ((s_11_17) == (s_11_19));
        // D s_11_21: write-var index <= s_11_20
        fn_state.index = s_11_20;
        // D s_11_22: read-var U:u8
        let s_11_22: bool = fn_state.U;
        // D s_11_23: cast zx s_11_22 -> bv
        let s_11_23: Bits = Bits::new(s_11_22 as u128, 1u16);
        // C s_11_24: const #1u : u8
        let s_11_24: bool = true;
        // C s_11_25: cast zx s_11_24 -> bv
        let s_11_25: Bits = Bits::new(s_11_24 as u128, 1u16);
        // D s_11_26: cmp-eq s_11_23 s_11_25
        let s_11_26: bool = ((s_11_23) == (s_11_25));
        // D s_11_27: write-var add <= s_11_26
        fn_state.add = s_11_26;
        // D s_11_28: read-var W:u8
        let s_11_28: bool = fn_state.W;
        // D s_11_29: cast zx s_11_28 -> bv
        let s_11_29: Bits = Bits::new(s_11_28 as u128, 1u16);
        // C s_11_30: const #1u : u8
        let s_11_30: bool = true;
        // C s_11_31: cast zx s_11_30 -> bv
        let s_11_31: Bits = Bits::new(s_11_30 as u128, 1u16);
        // D s_11_32: cmp-eq s_11_29 s_11_31
        let s_11_32: bool = ((s_11_29) == (s_11_31));
        // D s_11_33: write-var wback <= s_11_32
        fn_state.wback = s_11_32;
        // D s_11_34: read-var wback:u8
        let s_11_34: bool = fn_state.wback;
        // N s_11_35: branch s_11_34 b25 b12
        if s_11_34 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#297749 <= s_12_0
        fn_state.gs_297749 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#297749:u8
        let s_13_0: bool = fn_state.gs_297749;
        // N s_13_1: branch s_13_0 b24 b14
        if s_13_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #15s : i
        let s_14_0: i128 = 15;
        // D s_14_1: read-var t:i64
        let s_14_1: i64 = fn_state.t;
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (i128::try_from(s_14_1).unwrap());
        // D s_14_3: cmp-eq s_14_2 s_14_0
        let s_14_3: bool = ((s_14_2) == (s_14_0));
        // N s_14_4: branch s_14_3 b23 b15
        if s_14_3 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#297751 <= s_15_0
        fn_state.gs_297751 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#297751:u8
        let s_16_0: bool = fn_state.gs_297751;
        // N s_16_1: branch s_16_0 b22 b17
        if s_16_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#297752 <= s_17_0
        fn_state.gs_297752 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#297752:u8
        let s_18_0: bool = fn_state.gs_297752;
        // D s_18_1: write-var gs#297753 <= s_18_0
        fn_state.gs_297753 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#297753:u8
        let s_19_0: bool = fn_state.gs_297753;
        // N s_19_1: branch s_19_0 b21 b20
        if s_19_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var add:u8
        let s_20_0: bool = fn_state.add;
        // D s_20_1: read-var imm32:u32
        let s_20_1: u32 = fn_state.imm32;
        // D s_20_2: read-var index:u8
        let s_20_2: bool = fn_state.index;
        // D s_20_3: read-var n:i64
        let s_20_3: i64 = fn_state.n;
        // D s_20_4: read-var t:i64
        let s_20_4: i64 = fn_state.t;
        // D s_20_5: read-var wback:u8
        let s_20_5: bool = fn_state.wback;
        // D s_20_6: call execute_aarch32_instrs_LDR_i_OpT_A_txt(s_20_0, s_20_1, s_20_2, s_20_3, s_20_4, s_20_5)
        let s_20_6: () = execute_aarch32_instrs_LDR_i_OpT_A_txt(
            state,
            tracer,
            s_20_0,
            s_20_1,
            s_20_2,
            s_20_3,
            s_20_4,
            s_20_5,
        );
        // N s_20_7: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: panic
        panic!("{:?}", ());
        // N s_21_1: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call LastInITBlock(s_22_0)
        let s_22_1: bool = LastInITBlock(state, tracer, s_22_0);
        // S s_22_2: not s_22_1
        let s_22_2: bool = !s_22_1;
        // D s_22_3: write-var gs#297752 <= s_22_2
        fn_state.gs_297752 = s_22_2;
        // N s_22_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call InITBlock(s_23_0)
        let s_23_1: bool = InITBlock(state, tracer, s_23_0);
        // D s_23_2: write-var gs#297751 <= s_23_1
        fn_state.gs_297751 = s_23_1;
        // N s_23_3: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#297753 <= s_24_0
        fn_state.gs_297753 = s_24_0;
        // N s_24_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var n:i64
        let s_25_0: i64 = fn_state.n;
        // D s_25_1: cast zx s_25_0 -> i
        let s_25_1: i128 = (i128::try_from(s_25_0).unwrap());
        // D s_25_2: read-var t:i64
        let s_25_2: i64 = fn_state.t;
        // D s_25_3: cast zx s_25_2 -> i
        let s_25_3: i128 = (i128::try_from(s_25_2).unwrap());
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#297749 <= s_25_4
        fn_state.gs_297749 = s_25_4;
        // N s_25_6: jump b13
        return block_13(state, tracer, fn_state);
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
        // D s_27_0: read-var W:u8
        let s_27_0: bool = fn_state.W;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #0u : u8
        let s_27_2: bool = false;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#297742 <= s_27_4
        fn_state.gs_297742 = s_27_4;
        // N s_27_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: panic
        panic!("{:?}", ());
        // N s_28_1: return
        return;
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
        // C s_29_2: const #0u : u8
        let s_29_2: bool = false;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#297741 <= s_29_4
        fn_state.gs_297741 = s_29_4;
        // N s_29_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var U:u8
        let s_30_0: bool = fn_state.U;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 1u16);
        // C s_30_2: const #1u : u8
        let s_30_2: bool = true;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // D s_30_5: write-var gs#297740 <= s_30_4
        fn_state.gs_297740 = s_30_4;
        // N s_30_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_31_0: panic
        panic!("{:?}", ());
        // N s_31_1: return
        return;
    }
}
