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
use DecodeImmShift::*;
use execute_aarch32_instrs_LDRB_r_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_LDRB_r_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    P: bool,
    U: bool,
    W: bool,
    Rn: u8,
    Rt: u8,
    imm5: u8,
    stype: u8,
    Rm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        t: i64,
        gs_297091: bool,
        shift_nshadow_7152: i128,
        gs_297096: bool,
        n: i64,
        index: bool,
        gs_297104: bool,
        add: bool,
        ga_344017: ProductType396b95aa74979079,
        wback: bool,
        gs_297107: bool,
        shift_t: u32,
        gs_297106: bool,
        cond: u8,
        P: bool,
        U: bool,
        W: bool,
        Rn: u8,
        Rt: u8,
        imm5: u8,
        stype: u8,
        Rm: u8,
    }
    let fn_state = FunctionState {
        cond,
        P,
        U,
        W,
        Rn,
        Rt,
        imm5,
        stype,
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
        // N s_2_11: branch s_2_10 b23 b3
        if s_2_10 {
            return block_23(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#297091 <= s_3_0
        fn_state.gs_297091 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#297091:u8
        let s_4_0: bool = fn_state.gs_297091;
        // N s_4_1: branch s_4_0 b22 b5
        if s_4_0 {
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
        // D s_5_10: read-var Rm:u8
        let s_5_10: u8 = fn_state.Rm;
        // D s_5_11: cast zx s_5_10 -> bv
        let s_5_11: Bits = Bits::new(s_5_10 as u128, 4u16);
        // D s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (s_5_11.value() as i128);
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // D s_5_14: write-var m <= s_5_13
        fn_state.m = s_5_13;
        // D s_5_15: read-var P:u8
        let s_5_15: bool = fn_state.P;
        // D s_5_16: cast zx s_5_15 -> bv
        let s_5_16: Bits = Bits::new(s_5_15 as u128, 1u16);
        // C s_5_17: const #1u : u8
        let s_5_17: bool = true;
        // C s_5_18: cast zx s_5_17 -> bv
        let s_5_18: Bits = Bits::new(s_5_17 as u128, 1u16);
        // D s_5_19: cmp-eq s_5_16 s_5_18
        let s_5_19: bool = ((s_5_16) == (s_5_18));
        // D s_5_20: write-var index <= s_5_19
        fn_state.index = s_5_19;
        // D s_5_21: read-var U:u8
        let s_5_21: bool = fn_state.U;
        // D s_5_22: cast zx s_5_21 -> bv
        let s_5_22: Bits = Bits::new(s_5_21 as u128, 1u16);
        // C s_5_23: const #1u : u8
        let s_5_23: bool = true;
        // C s_5_24: cast zx s_5_23 -> bv
        let s_5_24: Bits = Bits::new(s_5_23 as u128, 1u16);
        // D s_5_25: cmp-eq s_5_22 s_5_24
        let s_5_25: bool = ((s_5_22) == (s_5_24));
        // D s_5_26: write-var add <= s_5_25
        fn_state.add = s_5_25;
        // D s_5_27: read-var P:u8
        let s_5_27: bool = fn_state.P;
        // D s_5_28: cast zx s_5_27 -> bv
        let s_5_28: Bits = Bits::new(s_5_27 as u128, 1u16);
        // C s_5_29: const #0u : u8
        let s_5_29: bool = false;
        // C s_5_30: cast zx s_5_29 -> bv
        let s_5_30: Bits = Bits::new(s_5_29 as u128, 1u16);
        // D s_5_31: cmp-eq s_5_28 s_5_30
        let s_5_31: bool = ((s_5_28) == (s_5_30));
        // N s_5_32: branch s_5_31 b21 b6
        if s_5_31 {
            return block_21(state, tracer, fn_state);
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
        // D s_6_5: write-var gs#297096 <= s_6_4
        fn_state.gs_297096 = s_6_4;
        // N s_6_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#297096:u8
        let s_7_0: bool = fn_state.gs_297096;
        // D s_7_1: write-var wback <= s_7_0
        fn_state.wback = s_7_0;
        // D s_7_2: read-var stype:u8
        let s_7_2: u8 = fn_state.stype;
        // D s_7_3: read-var imm5:u8
        let s_7_3: u8 = fn_state.imm5;
        // D s_7_4: call DecodeImmShift(s_7_2, s_7_3)
        let s_7_4: ProductType396b95aa74979079 = DecodeImmShift(
            state,
            tracer,
            s_7_2,
            s_7_3,
        );
        // D s_7_5: write-var ga#344017 <= s_7_4
        fn_state.ga_344017 = s_7_4;
        // D s_7_6: read-var ga#344017.0:struct
        let s_7_6: u32 = fn_state.ga_344017._0;
        // D s_7_7: read-var ga#344017.1:struct
        let s_7_7: i128 = fn_state.ga_344017._1;
        // D s_7_8: write-var shift_t <= s_7_6
        fn_state.shift_t = s_7_6;
        // D s_7_9: write-var shift_nshadow#7152 <= s_7_7
        fn_state.shift_nshadow_7152 = s_7_7;
        // C s_7_10: const #15s : i
        let s_7_10: i128 = 15;
        // D s_7_11: read-var t:i64
        let s_7_11: i64 = fn_state.t;
        // D s_7_12: cast zx s_7_11 -> i
        let s_7_12: i128 = (i128::try_from(s_7_11).unwrap());
        // D s_7_13: cmp-eq s_7_12 s_7_10
        let s_7_13: bool = ((s_7_12) == (s_7_10));
        // N s_7_14: branch s_7_13 b20 b8
        if s_7_13 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #15s : i
        let s_8_0: i128 = 15;
        // D s_8_1: read-var m:i64
        let s_8_1: i64 = fn_state.m;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: cmp-eq s_8_2 s_8_0
        let s_8_3: bool = ((s_8_2) == (s_8_0));
        // D s_8_4: write-var gs#297104 <= s_8_3
        fn_state.gs_297104 = s_8_3;
        // N s_8_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#297104:u8
        let s_9_0: bool = fn_state.gs_297104;
        // N s_9_1: branch s_9_0 b19 b10
        if s_9_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var wback:u8
        let s_10_0: bool = fn_state.wback;
        // N s_10_1: branch s_10_0 b15 b11
        if s_10_0 {
            return block_15(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#297107 <= s_11_0
        fn_state.gs_297107 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#297107:u8
        let s_12_0: bool = fn_state.gs_297107;
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
        // D s_13_1: read-var index:u8
        let s_13_1: bool = fn_state.index;
        // D s_13_2: read-var m:i64
        let s_13_2: i64 = fn_state.m;
        // D s_13_3: read-var n:i64
        let s_13_3: i64 = fn_state.n;
        // D s_13_4: read-var shift_nshadow#7152:i
        let s_13_4: i128 = fn_state.shift_nshadow_7152;
        // D s_13_5: read-var shift_t:u32
        let s_13_5: u32 = fn_state.shift_t;
        // D s_13_6: read-var t:i64
        let s_13_6: i64 = fn_state.t;
        // D s_13_7: read-var wback:u8
        let s_13_7: bool = fn_state.wback;
        // D s_13_8: call execute_aarch32_instrs_LDRB_r_Op_A_txt(s_13_0, s_13_1, s_13_2, s_13_3, s_13_4, s_13_5, s_13_6, s_13_7)
        let s_13_8: () = execute_aarch32_instrs_LDRB_r_Op_A_txt(
            state,
            tracer,
            s_13_0,
            s_13_1,
            s_13_2,
            s_13_3,
            s_13_4,
            s_13_5,
            s_13_6,
            s_13_7,
        );
        // N s_13_9: return
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
        // N s_15_4: branch s_15_3 b18 b16
        if s_15_3 {
            return block_18(state, tracer, fn_state);
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
        // D s_16_5: write-var gs#297106 <= s_16_4
        fn_state.gs_297106 = s_16_4;
        // N s_16_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#297106:u8
        let s_17_0: bool = fn_state.gs_297106;
        // D s_17_1: write-var gs#297107 <= s_17_0
        fn_state.gs_297107 = s_17_0;
        // N s_17_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#297106 <= s_18_0
        fn_state.gs_297106 = s_18_0;
        // N s_18_2: jump b17
        return block_17(state, tracer, fn_state);
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
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#297104 <= s_20_0
        fn_state.gs_297104 = s_20_0;
        // N s_20_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#297096 <= s_21_0
        fn_state.gs_297096 = s_21_0;
        // N s_21_2: jump b7
        return block_7(state, tracer, fn_state);
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
        // D s_23_5: write-var gs#297091 <= s_23_4
        fn_state.gs_297091 = s_23_4;
        // N s_23_6: jump b4
        return block_4(state, tracer, fn_state);
    }
}
