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
use execute_aarch32_instrs_STRD_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_STRD_i_A1enc_A_txt<T: Tracer>(
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
        t2: i64,
        gs_302514: bool,
        gs_302515: bool,
        gs_302510: bool,
        n: i64,
        index: bool,
        add: bool,
        gs_302511: bool,
        wback: bool,
        gs_302513: bool,
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
        // C s_2_6: const #0s : i
        let s_2_6: i128 = 0;
        // D s_2_7: read-var Rt:u8
        let s_2_7: u8 = fn_state.Rt;
        // D s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 4u16);
        // C s_2_9: const #1u : u64
        let s_2_9: u64 = 1;
        // D s_2_10: bit-extract s_2_8 s_2_6 s_2_9
        let s_2_10: Bits = (Bits::new(
            ((s_2_8) >> (s_2_6)).value(),
            u16::try_from(s_2_9).unwrap(),
        ));
        // D s_2_11: cast reint s_2_10 -> u8
        let s_2_11: bool = ((s_2_10.value()) != 0);
        // C s_2_12: const #0s : i
        let s_2_12: i128 = 0;
        // C s_2_13: const #0u : u64
        let s_2_13: u64 = 0;
        // D s_2_14: cast zx s_2_11 -> u64
        let s_2_14: u64 = (s_2_11 as u64);
        // C s_2_15: const #1u : u64
        let s_2_15: u64 = 1;
        // D s_2_16: and s_2_14 s_2_15
        let s_2_16: u64 = ((s_2_14) & (s_2_15));
        // D s_2_17: cmp-eq s_2_16 s_2_15
        let s_2_17: bool = ((s_2_16) == (s_2_15));
        // D s_2_18: lsl s_2_14 s_2_12
        let s_2_18: u64 = s_2_14 << s_2_12;
        // D s_2_19: or s_2_13 s_2_18
        let s_2_19: u64 = ((s_2_13) | (s_2_18));
        // D s_2_20: cmpl s_2_18
        let s_2_20: u64 = !s_2_18;
        // D s_2_21: and s_2_13 s_2_20
        let s_2_21: u64 = ((s_2_13) & (s_2_20));
        // D s_2_22: select s_2_17 s_2_19 s_2_21
        let s_2_22: u64 = if s_2_17 { s_2_19 } else { s_2_21 };
        // D s_2_23: cast trunc s_2_22 -> u8
        let s_2_23: bool = ((s_2_22) != 0);
        // D s_2_24: cast zx s_2_23 -> bv
        let s_2_24: Bits = Bits::new(s_2_23 as u128, 1u16);
        // C s_2_25: const #1u : u8
        let s_2_25: bool = true;
        // C s_2_26: cast zx s_2_25 -> bv
        let s_2_26: Bits = Bits::new(s_2_25 as u128, 1u16);
        // D s_2_27: cmp-eq s_2_24 s_2_26
        let s_2_27: bool = ((s_2_24) == (s_2_26));
        // N s_2_28: branch s_2_27 b25 b3
        if s_2_27 {
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
        // D s_3_0: read-var Rt:u8
        let s_3_0: u8 = fn_state.Rt;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 4u16);
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // D s_3_4: write-var t <= s_3_3
        fn_state.t = s_3_3;
        // C s_3_5: const #1s : i
        let s_3_5: i128 = 1;
        // D s_3_6: read-var t:i64
        let s_3_6: i64 = fn_state.t;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: add s_3_7 s_3_5
        let s_3_8: i128 = (s_3_7 + s_3_5);
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // D s_3_10: write-var t2 <= s_3_9
        fn_state.t2 = s_3_9;
        // D s_3_11: read-var Rn:u8
        let s_3_11: u8 = fn_state.Rn;
        // D s_3_12: cast zx s_3_11 -> bv
        let s_3_12: Bits = Bits::new(s_3_11 as u128, 4u16);
        // D s_3_13: cast zx s_3_12 -> i
        let s_3_13: i128 = (s_3_12.value() as i128);
        // D s_3_14: cast reint s_3_13 -> i64
        let s_3_14: i64 = (s_3_13 as i64);
        // D s_3_15: write-var n <= s_3_14
        fn_state.n = s_3_14;
        // D s_3_16: read-var imm4H:u8
        let s_3_16: u8 = fn_state.imm4H;
        // D s_3_17: cast zx s_3_16 -> bv
        let s_3_17: Bits = Bits::new(s_3_16 as u128, 4u16);
        // D s_3_18: read-var imm4L:u8
        let s_3_18: u8 = fn_state.imm4L;
        // D s_3_19: cast zx s_3_18 -> bv
        let s_3_19: Bits = Bits::new(s_3_18 as u128, 4u16);
        // D s_3_20: cast reint s_3_17 -> u128
        let s_3_20: u128 = (s_3_17.value() as u128);
        // D s_3_21: size-of s_3_17
        let s_3_21: u16 = s_3_17.length();
        // D s_3_22: cast reint s_3_19 -> u128
        let s_3_22: u128 = (s_3_19.value() as u128);
        // D s_3_23: size-of s_3_19
        let s_3_23: u16 = s_3_19.length();
        // D s_3_24: lsl s_3_20 s_3_23
        let s_3_24: u128 = s_3_20 << s_3_23;
        // D s_3_25: or s_3_24 s_3_22
        let s_3_25: u128 = ((s_3_24) | (s_3_22));
        // D s_3_26: add s_3_21 s_3_23
        let s_3_26: u16 = (s_3_21 + s_3_23);
        // D s_3_27: create-bits s_3_25 s_3_26
        let s_3_27: Bits = Bits::new(s_3_25, s_3_26);
        // D s_3_28: cast reint s_3_27 -> u8
        let s_3_28: u8 = (s_3_27.value() as u8);
        // C s_3_29: const #32s : i
        let s_3_29: i128 = 32;
        // D s_3_30: cast zx s_3_28 -> bv
        let s_3_30: Bits = Bits::new(s_3_28 as u128, 8u16);
        // D s_3_31: bits-cast zx s_3_30 -> bv length s_3_29
        let s_3_31: Bits = s_3_30.zero_extend(s_3_29);
        // D s_3_32: cast reint s_3_31 -> u32
        let s_3_32: u32 = (s_3_31.value() as u32);
        // D s_3_33: write-var imm32 <= s_3_32
        fn_state.imm32 = s_3_32;
        // D s_3_34: read-var P:u8
        let s_3_34: bool = fn_state.P;
        // D s_3_35: cast zx s_3_34 -> bv
        let s_3_35: Bits = Bits::new(s_3_34 as u128, 1u16);
        // C s_3_36: const #1u : u8
        let s_3_36: bool = true;
        // C s_3_37: cast zx s_3_36 -> bv
        let s_3_37: Bits = Bits::new(s_3_36 as u128, 1u16);
        // D s_3_38: cmp-eq s_3_35 s_3_37
        let s_3_38: bool = ((s_3_35) == (s_3_37));
        // D s_3_39: write-var index <= s_3_38
        fn_state.index = s_3_38;
        // D s_3_40: read-var U:u8
        let s_3_40: bool = fn_state.U;
        // D s_3_41: cast zx s_3_40 -> bv
        let s_3_41: Bits = Bits::new(s_3_40 as u128, 1u16);
        // C s_3_42: const #1u : u8
        let s_3_42: bool = true;
        // C s_3_43: cast zx s_3_42 -> bv
        let s_3_43: Bits = Bits::new(s_3_42 as u128, 1u16);
        // D s_3_44: cmp-eq s_3_41 s_3_43
        let s_3_44: bool = ((s_3_41) == (s_3_43));
        // D s_3_45: write-var add <= s_3_44
        fn_state.add = s_3_44;
        // D s_3_46: read-var P:u8
        let s_3_46: bool = fn_state.P;
        // D s_3_47: cast zx s_3_46 -> bv
        let s_3_47: Bits = Bits::new(s_3_46 as u128, 1u16);
        // C s_3_48: const #0u : u8
        let s_3_48: bool = false;
        // C s_3_49: cast zx s_3_48 -> bv
        let s_3_49: Bits = Bits::new(s_3_48 as u128, 1u16);
        // D s_3_50: cmp-eq s_3_47 s_3_49
        let s_3_50: bool = ((s_3_47) == (s_3_49));
        // N s_3_51: branch s_3_50 b24 b4
        if s_3_50 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var W:u8
        let s_4_0: bool = fn_state.W;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 1u16);
        // C s_4_2: const #1u : u8
        let s_4_2: bool = true;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // D s_4_5: write-var gs#302510 <= s_4_4
        fn_state.gs_302510 = s_4_4;
        // N s_4_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#302510:u8
        let s_5_0: bool = fn_state.gs_302510;
        // D s_5_1: write-var wback <= s_5_0
        fn_state.wback = s_5_0;
        // D s_5_2: read-var P:u8
        let s_5_2: bool = fn_state.P;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // C s_5_4: const #0u : u8
        let s_5_4: bool = false;
        // C s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 1u16);
        // D s_5_6: cmp-eq s_5_3 s_5_5
        let s_5_6: bool = ((s_5_3) == (s_5_5));
        // N s_5_7: branch s_5_6 b23 b6
        if s_5_6 {
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
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#302511 <= s_6_0
        fn_state.gs_302511 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#302511:u8
        let s_7_0: bool = fn_state.gs_302511;
        // N s_7_1: branch s_7_0 b22 b8
        if s_7_0 {
            return block_22(state, tracer, fn_state);
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
        // N s_8_1: branch s_8_0 b15 b9
        if s_8_0 {
            return block_15(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#302515 <= s_9_0
        fn_state.gs_302515 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#302515:u8
        let s_10_0: bool = fn_state.gs_302515;
        // N s_10_1: branch s_10_0 b14 b11
        if s_10_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #15s : i
        let s_11_0: i128 = 15;
        // D s_11_1: read-var t2:i64
        let s_11_1: i64 = fn_state.t2;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: cmp-eq s_11_2 s_11_0
        let s_11_3: bool = ((s_11_2) == (s_11_0));
        // N s_11_4: branch s_11_3 b13 b12
        if s_11_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var add:u8
        let s_12_0: bool = fn_state.add;
        // D s_12_1: read-var imm32:u32
        let s_12_1: u32 = fn_state.imm32;
        // D s_12_2: read-var index:u8
        let s_12_2: bool = fn_state.index;
        // D s_12_3: read-var n:i64
        let s_12_3: i64 = fn_state.n;
        // D s_12_4: read-var t:i64
        let s_12_4: i64 = fn_state.t;
        // D s_12_5: read-var t2:i64
        let s_12_5: i64 = fn_state.t2;
        // D s_12_6: read-var wback:u8
        let s_12_6: bool = fn_state.wback;
        // D s_12_7: call execute_aarch32_instrs_STRD_i_Op_A_txt(s_12_0, s_12_1, s_12_2, s_12_3, s_12_4, s_12_5, s_12_6)
        let s_12_7: () = execute_aarch32_instrs_STRD_i_Op_A_txt(
            state,
            tracer,
            s_12_0,
            s_12_1,
            s_12_2,
            s_12_3,
            s_12_4,
            s_12_5,
            s_12_6,
        );
        // N s_12_8: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: panic
        panic!("{:?}", ());
        // N s_13_1: return
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
        // C s_15_0: const #15s : i
        let s_15_0: i128 = 15;
        // D s_15_1: read-var n:i64
        let s_15_1: i64 = fn_state.n;
        // D s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (i128::try_from(s_15_1).unwrap());
        // D s_15_3: cmp-eq s_15_2 s_15_0
        let s_15_3: bool = ((s_15_2) == (s_15_0));
        // N s_15_4: branch s_15_3 b21 b16
        if s_15_3 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var n:i64
        let s_16_0: i64 = fn_state.n;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: read-var t:i64
        let s_16_2: i64 = fn_state.t;
        // D s_16_3: cast zx s_16_2 -> i
        let s_16_3: i128 = (i128::try_from(s_16_2).unwrap());
        // D s_16_4: cmp-eq s_16_1 s_16_3
        let s_16_4: bool = ((s_16_1) == (s_16_3));
        // D s_16_5: write-var gs#302513 <= s_16_4
        fn_state.gs_302513 = s_16_4;
        // N s_16_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#302513:u8
        let s_17_0: bool = fn_state.gs_302513;
        // N s_17_1: branch s_17_0 b20 b18
        if s_17_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var n:i64
        let s_18_0: i64 = fn_state.n;
        // D s_18_1: cast zx s_18_0 -> i
        let s_18_1: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_2: read-var t2:i64
        let s_18_2: i64 = fn_state.t2;
        // D s_18_3: cast zx s_18_2 -> i
        let s_18_3: i128 = (i128::try_from(s_18_2).unwrap());
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // D s_18_5: write-var gs#302514 <= s_18_4
        fn_state.gs_302514 = s_18_4;
        // N s_18_6: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#302514:u8
        let s_19_0: bool = fn_state.gs_302514;
        // D s_19_1: write-var gs#302515 <= s_19_0
        fn_state.gs_302515 = s_19_0;
        // N s_19_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#302514 <= s_20_0
        fn_state.gs_302514 = s_20_0;
        // N s_20_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#302513 <= s_21_0
        fn_state.gs_302513 = s_21_0;
        // N s_21_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: panic
        panic!("{:?}", ());
        // N s_22_1: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var W:u8
        let s_23_0: bool = fn_state.W;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 1u16);
        // C s_23_2: const #1u : u8
        let s_23_2: bool = true;
        // C s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // D s_23_4: cmp-eq s_23_1 s_23_3
        let s_23_4: bool = ((s_23_1) == (s_23_3));
        // D s_23_5: write-var gs#302511 <= s_23_4
        fn_state.gs_302511 = s_23_4;
        // N s_23_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#302510 <= s_24_0
        fn_state.gs_302510 = s_24_0;
        // N s_24_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: panic
        panic!("{:?}", ());
        // N s_25_1: return
        return;
    }
}
