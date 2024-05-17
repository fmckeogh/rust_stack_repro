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
use execute_aarch32_instrs_LDRD_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_LDRD_i_A1enc_A_txt<T: Tracer>(
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
        t2: i64,
        imm32: u32,
        n: i64,
        index: bool,
        add: bool,
        gs_297233: bool,
        wback: bool,
        gs_297232: bool,
        gs_297230: bool,
        gs_297231: bool,
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
        // N s_2_11: branch s_2_10 b24 b3
        if s_2_10 {
            return block_24(state, tracer, fn_state);
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
        // D s_3_1: read-var Rt:u8
        let s_3_1: u8 = fn_state.Rt;
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
        // N s_3_22: branch s_3_21 b23 b4
        if s_3_21 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var Rt:u8
        let s_4_0: u8 = fn_state.Rt;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 4u16);
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (s_4_1.value() as i128);
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // D s_4_4: write-var t <= s_4_3
        fn_state.t = s_4_3;
        // C s_4_5: const #1s : i
        let s_4_5: i128 = 1;
        // D s_4_6: read-var t:i64
        let s_4_6: i64 = fn_state.t;
        // D s_4_7: cast zx s_4_6 -> i
        let s_4_7: i128 = (i128::try_from(s_4_6).unwrap());
        // D s_4_8: add s_4_7 s_4_5
        let s_4_8: i128 = (s_4_7 + s_4_5);
        // D s_4_9: cast reint s_4_8 -> i64
        let s_4_9: i64 = (s_4_8 as i64);
        // D s_4_10: write-var t2 <= s_4_9
        fn_state.t2 = s_4_9;
        // D s_4_11: read-var Rn:u8
        let s_4_11: u8 = fn_state.Rn;
        // D s_4_12: cast zx s_4_11 -> bv
        let s_4_12: Bits = Bits::new(s_4_11 as u128, 4u16);
        // D s_4_13: cast zx s_4_12 -> i
        let s_4_13: i128 = (s_4_12.value() as i128);
        // D s_4_14: cast reint s_4_13 -> i64
        let s_4_14: i64 = (s_4_13 as i64);
        // D s_4_15: write-var n <= s_4_14
        fn_state.n = s_4_14;
        // D s_4_16: read-var imm4H:u8
        let s_4_16: u8 = fn_state.imm4H;
        // D s_4_17: cast zx s_4_16 -> bv
        let s_4_17: Bits = Bits::new(s_4_16 as u128, 4u16);
        // D s_4_18: read-var imm4L:u8
        let s_4_18: u8 = fn_state.imm4L;
        // D s_4_19: cast zx s_4_18 -> bv
        let s_4_19: Bits = Bits::new(s_4_18 as u128, 4u16);
        // D s_4_20: cast reint s_4_17 -> u128
        let s_4_20: u128 = (s_4_17.value() as u128);
        // D s_4_21: size-of s_4_17
        let s_4_21: u16 = s_4_17.length();
        // D s_4_22: cast reint s_4_19 -> u128
        let s_4_22: u128 = (s_4_19.value() as u128);
        // D s_4_23: size-of s_4_19
        let s_4_23: u16 = s_4_19.length();
        // D s_4_24: lsl s_4_20 s_4_23
        let s_4_24: u128 = s_4_20 << s_4_23;
        // D s_4_25: or s_4_24 s_4_22
        let s_4_25: u128 = ((s_4_24) | (s_4_22));
        // D s_4_26: add s_4_21 s_4_23
        let s_4_26: u16 = (s_4_21 + s_4_23);
        // D s_4_27: create-bits s_4_25 s_4_26
        let s_4_27: Bits = Bits::new(s_4_25, s_4_26);
        // D s_4_28: cast reint s_4_27 -> u8
        let s_4_28: u8 = (s_4_27.value() as u8);
        // C s_4_29: const #32s : i
        let s_4_29: i128 = 32;
        // D s_4_30: cast zx s_4_28 -> bv
        let s_4_30: Bits = Bits::new(s_4_28 as u128, 8u16);
        // D s_4_31: bits-cast zx s_4_30 -> bv length s_4_29
        let s_4_31: Bits = s_4_30.zero_extend(s_4_29);
        // D s_4_32: cast reint s_4_31 -> u32
        let s_4_32: u32 = (s_4_31.value() as u32);
        // D s_4_33: write-var imm32 <= s_4_32
        fn_state.imm32 = s_4_32;
        // D s_4_34: read-var P:u8
        let s_4_34: bool = fn_state.P;
        // D s_4_35: cast zx s_4_34 -> bv
        let s_4_35: Bits = Bits::new(s_4_34 as u128, 1u16);
        // C s_4_36: const #1u : u8
        let s_4_36: bool = true;
        // C s_4_37: cast zx s_4_36 -> bv
        let s_4_37: Bits = Bits::new(s_4_36 as u128, 1u16);
        // D s_4_38: cmp-eq s_4_35 s_4_37
        let s_4_38: bool = ((s_4_35) == (s_4_37));
        // D s_4_39: write-var index <= s_4_38
        fn_state.index = s_4_38;
        // D s_4_40: read-var U:u8
        let s_4_40: bool = fn_state.U;
        // D s_4_41: cast zx s_4_40 -> bv
        let s_4_41: Bits = Bits::new(s_4_40 as u128, 1u16);
        // C s_4_42: const #1u : u8
        let s_4_42: bool = true;
        // C s_4_43: cast zx s_4_42 -> bv
        let s_4_43: Bits = Bits::new(s_4_42 as u128, 1u16);
        // D s_4_44: cmp-eq s_4_41 s_4_43
        let s_4_44: bool = ((s_4_41) == (s_4_43));
        // D s_4_45: write-var add <= s_4_44
        fn_state.add = s_4_44;
        // D s_4_46: read-var P:u8
        let s_4_46: bool = fn_state.P;
        // D s_4_47: cast zx s_4_46 -> bv
        let s_4_47: Bits = Bits::new(s_4_46 as u128, 1u16);
        // C s_4_48: const #0u : u8
        let s_4_48: bool = false;
        // C s_4_49: cast zx s_4_48 -> bv
        let s_4_49: Bits = Bits::new(s_4_48 as u128, 1u16);
        // D s_4_50: cmp-eq s_4_47 s_4_49
        let s_4_50: bool = ((s_4_47) == (s_4_49));
        // N s_4_51: branch s_4_50 b22 b5
        if s_4_50 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var W:u8
        let s_5_0: bool = fn_state.W;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #1u : u8
        let s_5_2: bool = true;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: write-var gs#297230 <= s_5_4
        fn_state.gs_297230 = s_5_4;
        // N s_5_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#297230:u8
        let s_6_0: bool = fn_state.gs_297230;
        // D s_6_1: write-var wback <= s_6_0
        fn_state.wback = s_6_0;
        // D s_6_2: read-var P:u8
        let s_6_2: bool = fn_state.P;
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // C s_6_4: const #0u : u8
        let s_6_4: bool = false;
        // C s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 1u16);
        // D s_6_6: cmp-eq s_6_3 s_6_5
        let s_6_6: bool = ((s_6_3) == (s_6_5));
        // N s_6_7: branch s_6_6 b21 b7
        if s_6_6 {
            return block_21(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#297231 <= s_7_0
        fn_state.gs_297231 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#297231:u8
        let s_8_0: bool = fn_state.gs_297231;
        // N s_8_1: branch s_8_0 b20 b9
        if s_8_0 {
            return block_20(state, tracer, fn_state);
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
        // N s_9_1: branch s_9_0 b16 b10
        if s_9_0 {
            return block_16(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#297233 <= s_10_0
        fn_state.gs_297233 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#297233:u8
        let s_11_0: bool = fn_state.gs_297233;
        // N s_11_1: branch s_11_0 b15 b12
        if s_11_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #15s : i
        let s_12_0: i128 = 15;
        // D s_12_1: read-var t2:i64
        let s_12_1: i64 = fn_state.t2;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: cmp-eq s_12_2 s_12_0
        let s_12_3: bool = ((s_12_2) == (s_12_0));
        // N s_12_4: branch s_12_3 b14 b13
        if s_12_3 {
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
        // D s_13_5: read-var t2:i64
        let s_13_5: i64 = fn_state.t2;
        // D s_13_6: read-var wback:u8
        let s_13_6: bool = fn_state.wback;
        // D s_13_7: call execute_aarch32_instrs_LDRD_i_Op_A_txt(s_13_0, s_13_1, s_13_2, s_13_3, s_13_4, s_13_5, s_13_6)
        let s_13_7: () = execute_aarch32_instrs_LDRD_i_Op_A_txt(
            state,
            tracer,
            s_13_0,
            s_13_1,
            s_13_2,
            s_13_3,
            s_13_4,
            s_13_5,
            s_13_6,
        );
        // N s_13_8: return
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
        // N s_16_5: branch s_16_4 b19 b17
        if s_16_4 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var n:i64
        let s_17_0: i64 = fn_state.n;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: read-var t2:i64
        let s_17_2: i64 = fn_state.t2;
        // D s_17_3: cast zx s_17_2 -> i
        let s_17_3: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // D s_17_5: write-var gs#297232 <= s_17_4
        fn_state.gs_297232 = s_17_4;
        // N s_17_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#297232:u8
        let s_18_0: bool = fn_state.gs_297232;
        // D s_18_1: write-var gs#297233 <= s_18_0
        fn_state.gs_297233 = s_18_0;
        // N s_18_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#297232 <= s_19_0
        fn_state.gs_297232 = s_19_0;
        // N s_19_2: jump b18
        return block_18(state, tracer, fn_state);
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
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var W:u8
        let s_21_0: bool = fn_state.W;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 1u16);
        // C s_21_2: const #1u : u8
        let s_21_2: bool = true;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // D s_21_4: cmp-eq s_21_1 s_21_3
        let s_21_4: bool = ((s_21_1) == (s_21_3));
        // D s_21_5: write-var gs#297231 <= s_21_4
        fn_state.gs_297231 = s_21_4;
        // N s_21_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#297230 <= s_22_0
        fn_state.gs_297230 = s_22_0;
        // N s_22_2: jump b6
        return block_6(state, tracer, fn_state);
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
}
