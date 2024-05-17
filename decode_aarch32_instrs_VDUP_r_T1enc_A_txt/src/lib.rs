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
use execute_aarch32_instrs_VDUP_r_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VDUP_r_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    B: bool,
    Q: bool,
    Vd: u8,
    Rt: u8,
    D: bool,
    E: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        t: i64,
        ga_352745: u8,
        elementsshadow_7445: i64,
        regs: i64,
        esize: i64,
        d: i64,
        elements: i64,
        gs_308609: bool,
        esizeshadow_7444: i64,
        ga_352744: i64,
        B: bool,
        Q: bool,
        Vd: u8,
        Rt: u8,
        D: bool,
        E: bool,
    }
    let fn_state = FunctionState {
        B,
        Q,
        Vd,
        Rt,
        D,
        E,
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
        // D s_2_0: read-var Q:u8
        let s_2_0: bool = fn_state.Q;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #1u : u8
        let s_2_2: bool = true;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b19 b3
        if s_2_4 {
            return block_19(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#308609 <= s_3_0
        fn_state.gs_308609 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#308609:u8
        let s_4_0: bool = fn_state.gs_308609;
        // N s_4_1: branch s_4_0 b18 b5
        if s_4_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var D:u8
        let s_5_0: bool = fn_state.D;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // D s_5_2: read-var Vd:u8
        let s_5_2: u8 = fn_state.Vd;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 4u16);
        // D s_5_4: cast reint s_5_1 -> u128
        let s_5_4: u128 = (s_5_1.value() as u128);
        // D s_5_5: size-of s_5_1
        let s_5_5: u16 = s_5_1.length();
        // D s_5_6: cast reint s_5_3 -> u128
        let s_5_6: u128 = (s_5_3.value() as u128);
        // D s_5_7: size-of s_5_3
        let s_5_7: u16 = s_5_3.length();
        // D s_5_8: lsl s_5_4 s_5_7
        let s_5_8: u128 = s_5_4 << s_5_7;
        // D s_5_9: or s_5_8 s_5_6
        let s_5_9: u128 = ((s_5_8) | (s_5_6));
        // D s_5_10: add s_5_5 s_5_7
        let s_5_10: u16 = (s_5_5 + s_5_7);
        // D s_5_11: create-bits s_5_9 s_5_10
        let s_5_11: Bits = Bits::new(s_5_9, s_5_10);
        // D s_5_12: cast reint s_5_11 -> u8
        let s_5_12: u8 = (s_5_11.value() as u8);
        // D s_5_13: cast zx s_5_12 -> bv
        let s_5_13: Bits = Bits::new(s_5_12 as u128, 5u16);
        // D s_5_14: cast zx s_5_13 -> i
        let s_5_14: i128 = (s_5_13.value() as i128);
        // D s_5_15: cast reint s_5_14 -> i64
        let s_5_15: i64 = (s_5_14 as i64);
        // D s_5_16: write-var d <= s_5_15
        fn_state.d = s_5_15;
        // D s_5_17: read-var Rt:u8
        let s_5_17: u8 = fn_state.Rt;
        // D s_5_18: cast zx s_5_17 -> bv
        let s_5_18: Bits = Bits::new(s_5_17 as u128, 4u16);
        // D s_5_19: cast zx s_5_18 -> i
        let s_5_19: i128 = (s_5_18.value() as i128);
        // D s_5_20: cast reint s_5_19 -> i64
        let s_5_20: i64 = (s_5_19 as i64);
        // D s_5_21: write-var t <= s_5_20
        fn_state.t = s_5_20;
        // D s_5_22: read-var Q:u8
        let s_5_22: bool = fn_state.Q;
        // D s_5_23: cast zx s_5_22 -> bv
        let s_5_23: Bits = Bits::new(s_5_22 as u128, 1u16);
        // C s_5_24: const #0u : u8
        let s_5_24: bool = false;
        // C s_5_25: cast zx s_5_24 -> bv
        let s_5_25: Bits = Bits::new(s_5_24 as u128, 1u16);
        // D s_5_26: cmp-eq s_5_23 s_5_25
        let s_5_26: bool = ((s_5_23) == (s_5_25));
        // N s_5_27: branch s_5_26 b17 b6
        if s_5_26 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #2s : i64
        let s_6_0: i64 = 2;
        // D s_6_1: write-var ga#352744 <= s_6_0
        fn_state.ga_352744 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#352744:i64
        let s_7_0: i64 = fn_state.ga_352744;
        // D s_7_1: write-var regs <= s_7_0
        fn_state.regs = s_7_0;
        // C s_7_2: const #8s : i64
        let s_7_2: i64 = 8;
        // D s_7_3: write-var esize <= s_7_2
        fn_state.esize = s_7_2;
        // C s_7_4: const #2s : i64
        let s_7_4: i64 = 2;
        // D s_7_5: write-var elements <= s_7_4
        fn_state.elements = s_7_4;
        // D s_7_6: read-var B:u8
        let s_7_6: bool = fn_state.B;
        // D s_7_7: cast zx s_7_6 -> bv
        let s_7_7: Bits = Bits::new(s_7_6 as u128, 1u16);
        // D s_7_8: read-var E:u8
        let s_7_8: bool = fn_state.E;
        // D s_7_9: cast zx s_7_8 -> bv
        let s_7_9: Bits = Bits::new(s_7_8 as u128, 1u16);
        // D s_7_10: cast reint s_7_7 -> u128
        let s_7_10: u128 = (s_7_7.value() as u128);
        // D s_7_11: size-of s_7_7
        let s_7_11: u16 = s_7_7.length();
        // D s_7_12: cast reint s_7_9 -> u128
        let s_7_12: u128 = (s_7_9.value() as u128);
        // D s_7_13: size-of s_7_9
        let s_7_13: u16 = s_7_9.length();
        // D s_7_14: lsl s_7_10 s_7_13
        let s_7_14: u128 = s_7_10 << s_7_13;
        // D s_7_15: or s_7_14 s_7_12
        let s_7_15: u128 = ((s_7_14) | (s_7_12));
        // D s_7_16: add s_7_11 s_7_13
        let s_7_16: u16 = (s_7_11 + s_7_13);
        // D s_7_17: create-bits s_7_15 s_7_16
        let s_7_17: Bits = Bits::new(s_7_15, s_7_16);
        // D s_7_18: cast reint s_7_17 -> u8
        let s_7_18: u8 = (s_7_17.value() as u8);
        // D s_7_19: write-var ga#352745 <= s_7_18
        fn_state.ga_352745 = s_7_18;
        // D s_7_20: read-var ga#352745:u8
        let s_7_20: u8 = fn_state.ga_352745;
        // D s_7_21: cast zx s_7_20 -> bv
        let s_7_21: Bits = Bits::new(s_7_20 as u128, 2u16);
        // C s_7_22: const #0u : u8
        let s_7_22: u8 = 0;
        // C s_7_23: cast zx s_7_22 -> bv
        let s_7_23: Bits = Bits::new(s_7_22 as u128, 2u16);
        // D s_7_24: cmp-eq s_7_21 s_7_23
        let s_7_24: bool = ((s_7_21) == (s_7_23));
        // D s_7_25: not s_7_24
        let s_7_25: bool = !s_7_24;
        // N s_7_26: branch s_7_25 b12 b8
        if s_7_25 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #32s : i64
        let s_8_0: i64 = 32;
        // D s_8_1: write-var esize <= s_8_0
        fn_state.esize = s_8_0;
        // C s_8_2: const #2s : i64
        let s_8_2: i64 = 2;
        // D s_8_3: write-var elements <= s_8_2
        fn_state.elements = s_8_2;
        // N s_8_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var esize:i64
        let s_9_0: i64 = fn_state.esize;
        // D s_9_1: write-var esizeshadow#7444 <= s_9_0
        fn_state.esizeshadow_7444 = s_9_0;
        // D s_9_2: read-var elements:i64
        let s_9_2: i64 = fn_state.elements;
        // D s_9_3: write-var elementsshadow#7445 <= s_9_2
        fn_state.elementsshadow_7445 = s_9_2;
        // C s_9_4: const #15s : i
        let s_9_4: i128 = 15;
        // D s_9_5: read-var t:i64
        let s_9_5: i64 = fn_state.t;
        // D s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (i128::try_from(s_9_5).unwrap());
        // D s_9_7: cmp-eq s_9_6 s_9_4
        let s_9_7: bool = ((s_9_6) == (s_9_4));
        // N s_9_8: branch s_9_7 b11 b10
        if s_9_7 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var esizeshadow#7444:i64
        let s_10_0: i64 = fn_state.esizeshadow_7444;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var d:i64
        let s_10_3: i64 = fn_state.d;
        // D s_10_4: read-var elementsshadow#7445:i64
        let s_10_4: i64 = fn_state.elementsshadow_7445;
        // D s_10_5: read-var regs:i64
        let s_10_5: i64 = fn_state.regs;
        // D s_10_6: read-var t:i64
        let s_10_6: i64 = fn_state.t;
        // D s_10_7: call execute_aarch32_instrs_VDUP_r_Op_A_txt(s_10_3, s_10_4, s_10_2, s_10_5, s_10_6)
        let s_10_7: () = execute_aarch32_instrs_VDUP_r_Op_A_txt(
            state,
            tracer,
            s_10_3,
            s_10_4,
            s_10_2,
            s_10_5,
            s_10_6,
        );
        // N s_10_8: return
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
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var ga#352745:u8
        let s_12_0: u8 = fn_state.ga_352745;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 2u16);
        // C s_12_2: const #1u : u8
        let s_12_2: u8 = 1;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 2u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: not s_12_4
        let s_12_5: bool = !s_12_4;
        // N s_12_6: branch s_12_5 b14 b13
        if s_12_5 {
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
        // C s_13_0: const #16s : i64
        let s_13_0: i64 = 16;
        // D s_13_1: write-var esize <= s_13_0
        fn_state.esize = s_13_0;
        // C s_13_2: const #4s : i64
        let s_13_2: i64 = 4;
        // D s_13_3: write-var elements <= s_13_2
        fn_state.elements = s_13_2;
        // N s_13_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var ga#352745:u8
        let s_14_0: u8 = fn_state.ga_352745;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 2u16);
        // C s_14_2: const #2u : u8
        let s_14_2: u8 = 2;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 2u16);
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // D s_14_5: not s_14_4
        let s_14_5: bool = !s_14_4;
        // N s_14_6: branch s_14_5 b16 b15
        if s_14_5 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #8s : i64
        let s_15_0: i64 = 8;
        // D s_15_1: write-var esize <= s_15_0
        fn_state.esize = s_15_0;
        // C s_15_2: const #8s : i64
        let s_15_2: i64 = 8;
        // D s_15_3: write-var elements <= s_15_2
        fn_state.elements = s_15_2;
        // N s_15_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: panic
        panic!("{:?}", ());
        // N s_16_1: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1s : i64
        let s_17_0: i64 = 1;
        // D s_17_1: write-var ga#352744 <= s_17_0
        fn_state.ga_352744 = s_17_0;
        // N s_17_2: jump b7
        return block_7(state, tracer, fn_state);
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
        // C s_19_0: const #0s : i
        let s_19_0: i128 = 0;
        // D s_19_1: read-var Vd:u8
        let s_19_1: u8 = fn_state.Vd;
        // D s_19_2: cast zx s_19_1 -> bv
        let s_19_2: Bits = Bits::new(s_19_1 as u128, 4u16);
        // C s_19_3: const #1u : u64
        let s_19_3: u64 = 1;
        // D s_19_4: bit-extract s_19_2 s_19_0 s_19_3
        let s_19_4: Bits = (Bits::new(
            ((s_19_2) >> (s_19_0)).value(),
            u16::try_from(s_19_3).unwrap(),
        ));
        // D s_19_5: cast reint s_19_4 -> u8
        let s_19_5: bool = ((s_19_4.value()) != 0);
        // C s_19_6: const #0s : i
        let s_19_6: i128 = 0;
        // C s_19_7: const #0u : u64
        let s_19_7: u64 = 0;
        // D s_19_8: cast zx s_19_5 -> u64
        let s_19_8: u64 = (s_19_5 as u64);
        // C s_19_9: const #1u : u64
        let s_19_9: u64 = 1;
        // D s_19_10: and s_19_8 s_19_9
        let s_19_10: u64 = ((s_19_8) & (s_19_9));
        // D s_19_11: cmp-eq s_19_10 s_19_9
        let s_19_11: bool = ((s_19_10) == (s_19_9));
        // D s_19_12: lsl s_19_8 s_19_6
        let s_19_12: u64 = s_19_8 << s_19_6;
        // D s_19_13: or s_19_7 s_19_12
        let s_19_13: u64 = ((s_19_7) | (s_19_12));
        // D s_19_14: cmpl s_19_12
        let s_19_14: u64 = !s_19_12;
        // D s_19_15: and s_19_7 s_19_14
        let s_19_15: u64 = ((s_19_7) & (s_19_14));
        // D s_19_16: select s_19_11 s_19_13 s_19_15
        let s_19_16: u64 = if s_19_11 { s_19_13 } else { s_19_15 };
        // D s_19_17: cast trunc s_19_16 -> u8
        let s_19_17: bool = ((s_19_16) != 0);
        // D s_19_18: cast zx s_19_17 -> bv
        let s_19_18: Bits = Bits::new(s_19_17 as u128, 1u16);
        // C s_19_19: const #1u : u8
        let s_19_19: bool = true;
        // C s_19_20: cast zx s_19_19 -> bv
        let s_19_20: Bits = Bits::new(s_19_19 as u128, 1u16);
        // D s_19_21: cmp-eq s_19_18 s_19_20
        let s_19_21: bool = ((s_19_18) == (s_19_20));
        // D s_19_22: write-var gs#308609 <= s_19_21
        fn_state.gs_308609 = s_19_21;
        // N s_19_23: jump b4
        return block_4(state, tracer, fn_state);
    }
}
