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
use InITBlock::*;
use execute_aarch32_instrs_CPS_OpT_AS_txt::*;
use common::*;
pub fn decode_aarch32_instrs_CPS_T1enc_AS_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    im: bool,
    A: bool,
    I: bool,
    F: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        affectA: bool,
        disable: bool,
        affectI: bool,
        mode: u8,
        modeshadow_7883: u8,
        enable: bool,
        affectF: bool,
        im: bool,
        A: bool,
        I: bool,
        F: bool,
    }
    let fn_state = FunctionState {
        im,
        A,
        I,
        F,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var A:u8
        let s_0_0: bool = fn_state.A;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 1u16);
        // D s_0_2: read-var I:u8
        let s_0_2: bool = fn_state.I;
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // D s_0_4: cast reint s_0_1 -> u128
        let s_0_4: u128 = (s_0_1.value() as u128);
        // D s_0_5: size-of s_0_1
        let s_0_5: u16 = s_0_1.length();
        // D s_0_6: cast reint s_0_3 -> u128
        let s_0_6: u128 = (s_0_3.value() as u128);
        // D s_0_7: size-of s_0_3
        let s_0_7: u16 = s_0_3.length();
        // D s_0_8: lsl s_0_4 s_0_7
        let s_0_8: u128 = s_0_4 << s_0_7;
        // D s_0_9: or s_0_8 s_0_6
        let s_0_9: u128 = ((s_0_8) | (s_0_6));
        // D s_0_10: add s_0_5 s_0_7
        let s_0_10: u16 = (s_0_5 + s_0_7);
        // D s_0_11: create-bits s_0_9 s_0_10
        let s_0_11: Bits = Bits::new(s_0_9, s_0_10);
        // D s_0_12: cast reint s_0_11 -> u8
        let s_0_12: u8 = (s_0_11.value() as u8);
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 2u16);
        // D s_0_14: read-var F:u8
        let s_0_14: bool = fn_state.F;
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 1u16);
        // D s_0_16: cast reint s_0_13 -> u128
        let s_0_16: u128 = (s_0_13.value() as u128);
        // D s_0_17: size-of s_0_13
        let s_0_17: u16 = s_0_13.length();
        // D s_0_18: cast reint s_0_15 -> u128
        let s_0_18: u128 = (s_0_15.value() as u128);
        // D s_0_19: size-of s_0_15
        let s_0_19: u16 = s_0_15.length();
        // D s_0_20: lsl s_0_16 s_0_19
        let s_0_20: u128 = s_0_16 << s_0_19;
        // D s_0_21: or s_0_20 s_0_18
        let s_0_21: u128 = ((s_0_20) | (s_0_18));
        // D s_0_22: add s_0_17 s_0_19
        let s_0_22: u16 = (s_0_17 + s_0_19);
        // D s_0_23: create-bits s_0_21 s_0_22
        let s_0_23: Bits = Bits::new(s_0_21, s_0_22);
        // D s_0_24: cast reint s_0_23 -> u8
        let s_0_24: u8 = (s_0_23.value() as u8);
        // D s_0_25: cast zx s_0_24 -> bv
        let s_0_25: Bits = Bits::new(s_0_24 as u128, 3u16);
        // C s_0_26: const #0u : u8
        let s_0_26: u8 = 0;
        // C s_0_27: cast zx s_0_26 -> bv
        let s_0_27: Bits = Bits::new(s_0_26 as u128, 3u16);
        // D s_0_28: cmp-eq s_0_25 s_0_27
        let s_0_28: bool = ((s_0_25) == (s_0_27));
        // N s_0_29: branch s_0_28 b4 b1
        if s_0_28 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var mode:u8
        let s_1_0: u8 = fn_state.mode;
        // D s_1_1: write-var modeshadow#7883 <= s_1_0
        fn_state.modeshadow_7883 = s_1_0;
        // D s_1_2: read-var im:u8
        let s_1_2: bool = fn_state.im;
        // D s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 1u16);
        // C s_1_4: const #0u : u8
        let s_1_4: bool = false;
        // C s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 1u16);
        // D s_1_6: cmp-eq s_1_3 s_1_5
        let s_1_6: bool = ((s_1_3) == (s_1_5));
        // D s_1_7: write-var enable <= s_1_6
        fn_state.enable = s_1_6;
        // D s_1_8: read-var im:u8
        let s_1_8: bool = fn_state.im;
        // D s_1_9: cast zx s_1_8 -> bv
        let s_1_9: Bits = Bits::new(s_1_8 as u128, 1u16);
        // C s_1_10: const #1u : u8
        let s_1_10: bool = true;
        // C s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 1u16);
        // D s_1_12: cmp-eq s_1_9 s_1_11
        let s_1_12: bool = ((s_1_9) == (s_1_11));
        // D s_1_13: write-var disable <= s_1_12
        fn_state.disable = s_1_12;
        // D s_1_14: read-var A:u8
        let s_1_14: bool = fn_state.A;
        // D s_1_15: cast zx s_1_14 -> bv
        let s_1_15: Bits = Bits::new(s_1_14 as u128, 1u16);
        // C s_1_16: const #1u : u8
        let s_1_16: bool = true;
        // C s_1_17: cast zx s_1_16 -> bv
        let s_1_17: Bits = Bits::new(s_1_16 as u128, 1u16);
        // D s_1_18: cmp-eq s_1_15 s_1_17
        let s_1_18: bool = ((s_1_15) == (s_1_17));
        // D s_1_19: write-var affectA <= s_1_18
        fn_state.affectA = s_1_18;
        // D s_1_20: read-var I:u8
        let s_1_20: bool = fn_state.I;
        // D s_1_21: cast zx s_1_20 -> bv
        let s_1_21: Bits = Bits::new(s_1_20 as u128, 1u16);
        // C s_1_22: const #1u : u8
        let s_1_22: bool = true;
        // C s_1_23: cast zx s_1_22 -> bv
        let s_1_23: Bits = Bits::new(s_1_22 as u128, 1u16);
        // D s_1_24: cmp-eq s_1_21 s_1_23
        let s_1_24: bool = ((s_1_21) == (s_1_23));
        // D s_1_25: write-var affectI <= s_1_24
        fn_state.affectI = s_1_24;
        // D s_1_26: read-var F:u8
        let s_1_26: bool = fn_state.F;
        // D s_1_27: cast zx s_1_26 -> bv
        let s_1_27: Bits = Bits::new(s_1_26 as u128, 1u16);
        // C s_1_28: const #1u : u8
        let s_1_28: bool = true;
        // C s_1_29: cast zx s_1_28 -> bv
        let s_1_29: Bits = Bits::new(s_1_28 as u128, 1u16);
        // D s_1_30: cmp-eq s_1_27 s_1_29
        let s_1_30: bool = ((s_1_27) == (s_1_29));
        // D s_1_31: write-var affectF <= s_1_30
        fn_state.affectF = s_1_30;
        // C s_1_32: const #() : ()
        let s_1_32: () = ();
        // S s_1_33: call InITBlock(s_1_32)
        let s_1_33: bool = InITBlock(state, tracer, s_1_32);
        // N s_1_34: branch s_1_33 b3 b2
        if s_1_33 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var affectA:u8
        let s_2_0: bool = fn_state.affectA;
        // D s_2_1: read-var affectF:u8
        let s_2_1: bool = fn_state.affectF;
        // D s_2_2: read-var affectI:u8
        let s_2_2: bool = fn_state.affectI;
        // C s_2_3: const #0u : u8
        let s_2_3: bool = false;
        // D s_2_4: read-var disable:u8
        let s_2_4: bool = fn_state.disable;
        // D s_2_5: read-var enable:u8
        let s_2_5: bool = fn_state.enable;
        // D s_2_6: read-var modeshadow#7883:u8
        let s_2_6: u8 = fn_state.modeshadow_7883;
        // D s_2_7: call execute_aarch32_instrs_CPS_OpT_AS_txt(s_2_0, s_2_1, s_2_2, s_2_3, s_2_4, s_2_5, s_2_6)
        let s_2_7: () = execute_aarch32_instrs_CPS_OpT_AS_txt(
            state,
            tracer,
            s_2_0,
            s_2_1,
            s_2_2,
            s_2_3,
            s_2_4,
            s_2_5,
            s_2_6,
        );
        // N s_2_8: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: panic
        panic!("{:?}", ());
        // N s_3_1: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: panic
        panic!("{:?}", ());
        // N s_4_1: return
        return;
    }
}
