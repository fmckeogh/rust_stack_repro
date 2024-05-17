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
pub fn decode_aarch32_instrs_CPS_T2enc_AS_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imod: u8,
    M: bool,
    A: bool,
    I: bool,
    F: bool,
    mode: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        affectA: bool,
        gs_323195: bool,
        affectI: bool,
        gs_323194: bool,
        gs_323191: bool,
        gs_323187: bool,
        gs_323199: bool,
        enable: bool,
        changemode: bool,
        disable: bool,
        gs_323188: bool,
        affectF: bool,
        imod: u8,
        M: bool,
        A: bool,
        I: bool,
        F: bool,
        mode: u8,
    }
    let fn_state = FunctionState {
        imod,
        M,
        A,
        I,
        F,
        mode,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var imod:u8
        let s_0_0: u8 = fn_state.imod;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 2u16);
        // C s_0_2: const #0u : u8
        let s_0_2: u8 = 0;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 2u16);
        // D s_0_4: cmp-eq s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) == (s_0_3));
        // N s_0_5: branch s_0_4 b26 b1
        if s_0_4 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#323187 <= s_1_0
        fn_state.gs_323187 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#323187:u8
        let s_2_0: bool = fn_state.gs_323187;
        // N s_2_1: branch s_2_0 b25 b3
        if s_2_0 {
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
        // D s_3_0: read-var mode:u8
        let s_3_0: u8 = fn_state.mode;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 5u16);
        // C s_3_2: const #0u : u8
        let s_3_2: u8 = 0;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 5u16);
        // D s_3_4: cmp-ne s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) != (s_3_3));
        // N s_3_5: branch s_3_4 b24 b4
        if s_3_4 {
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
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#323188 <= s_4_0
        fn_state.gs_323188 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#323188:u8
        let s_5_0: bool = fn_state.gs_323188;
        // N s_5_1: branch s_5_0 b23 b6
        if s_5_0 {
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
        // C s_6_0: const #1s : i
        let s_6_0: i128 = 1;
        // D s_6_1: read-var imod:u8
        let s_6_1: u8 = fn_state.imod;
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 2u16);
        // C s_6_3: const #1u : u64
        let s_6_3: u64 = 1;
        // D s_6_4: bit-extract s_6_2 s_6_0 s_6_3
        let s_6_4: Bits = (Bits::new(
            ((s_6_2) >> (s_6_0)).value(),
            u16::try_from(s_6_3).unwrap(),
        ));
        // D s_6_5: cast reint s_6_4 -> u8
        let s_6_5: bool = ((s_6_4.value()) != 0);
        // C s_6_6: const #0s : i
        let s_6_6: i128 = 0;
        // C s_6_7: const #0u : u64
        let s_6_7: u64 = 0;
        // D s_6_8: cast zx s_6_5 -> u64
        let s_6_8: u64 = (s_6_5 as u64);
        // C s_6_9: const #1u : u64
        let s_6_9: u64 = 1;
        // D s_6_10: and s_6_8 s_6_9
        let s_6_10: u64 = ((s_6_8) & (s_6_9));
        // D s_6_11: cmp-eq s_6_10 s_6_9
        let s_6_11: bool = ((s_6_10) == (s_6_9));
        // D s_6_12: lsl s_6_8 s_6_6
        let s_6_12: u64 = s_6_8 << s_6_6;
        // D s_6_13: or s_6_7 s_6_12
        let s_6_13: u64 = ((s_6_7) | (s_6_12));
        // D s_6_14: cmpl s_6_12
        let s_6_14: u64 = !s_6_12;
        // D s_6_15: and s_6_7 s_6_14
        let s_6_15: u64 = ((s_6_7) & (s_6_14));
        // D s_6_16: select s_6_11 s_6_13 s_6_15
        let s_6_16: u64 = if s_6_11 { s_6_13 } else { s_6_15 };
        // D s_6_17: cast trunc s_6_16 -> u8
        let s_6_17: bool = ((s_6_16) != 0);
        // D s_6_18: cast zx s_6_17 -> bv
        let s_6_18: Bits = Bits::new(s_6_17 as u128, 1u16);
        // C s_6_19: const #1u : u8
        let s_6_19: bool = true;
        // C s_6_20: cast zx s_6_19 -> bv
        let s_6_20: Bits = Bits::new(s_6_19 as u128, 1u16);
        // D s_6_21: cmp-eq s_6_18 s_6_20
        let s_6_21: bool = ((s_6_18) == (s_6_20));
        // N s_6_22: branch s_6_21 b22 b7
        if s_6_21 {
            return block_22(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#323191 <= s_7_0
        fn_state.gs_323191 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#323191:u8
        let s_8_0: bool = fn_state.gs_323191;
        // N s_8_1: branch s_8_0 b21 b9
        if s_8_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1s : i
        let s_9_0: i128 = 1;
        // D s_9_1: read-var imod:u8
        let s_9_1: u8 = fn_state.imod;
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 2u16);
        // C s_9_3: const #1u : u64
        let s_9_3: u64 = 1;
        // D s_9_4: bit-extract s_9_2 s_9_0 s_9_3
        let s_9_4: Bits = (Bits::new(
            ((s_9_2) >> (s_9_0)).value(),
            u16::try_from(s_9_3).unwrap(),
        ));
        // D s_9_5: cast reint s_9_4 -> u8
        let s_9_5: bool = ((s_9_4.value()) != 0);
        // C s_9_6: const #0s : i
        let s_9_6: i128 = 0;
        // C s_9_7: const #0u : u64
        let s_9_7: u64 = 0;
        // D s_9_8: cast zx s_9_5 -> u64
        let s_9_8: u64 = (s_9_5 as u64);
        // C s_9_9: const #1u : u64
        let s_9_9: u64 = 1;
        // D s_9_10: and s_9_8 s_9_9
        let s_9_10: u64 = ((s_9_8) & (s_9_9));
        // D s_9_11: cmp-eq s_9_10 s_9_9
        let s_9_11: bool = ((s_9_10) == (s_9_9));
        // D s_9_12: lsl s_9_8 s_9_6
        let s_9_12: u64 = s_9_8 << s_9_6;
        // D s_9_13: or s_9_7 s_9_12
        let s_9_13: u64 = ((s_9_7) | (s_9_12));
        // D s_9_14: cmpl s_9_12
        let s_9_14: u64 = !s_9_12;
        // D s_9_15: and s_9_7 s_9_14
        let s_9_15: u64 = ((s_9_7) & (s_9_14));
        // D s_9_16: select s_9_11 s_9_13 s_9_15
        let s_9_16: u64 = if s_9_11 { s_9_13 } else { s_9_15 };
        // D s_9_17: cast trunc s_9_16 -> u8
        let s_9_17: bool = ((s_9_16) != 0);
        // D s_9_18: cast zx s_9_17 -> bv
        let s_9_18: Bits = Bits::new(s_9_17 as u128, 1u16);
        // C s_9_19: const #0u : u8
        let s_9_19: bool = false;
        // C s_9_20: cast zx s_9_19 -> bv
        let s_9_20: Bits = Bits::new(s_9_19 as u128, 1u16);
        // D s_9_21: cmp-eq s_9_18 s_9_20
        let s_9_21: bool = ((s_9_18) == (s_9_20));
        // N s_9_22: branch s_9_21 b20 b10
        if s_9_21 {
            return block_20(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#323194 <= s_10_0
        fn_state.gs_323194 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#323194:u8
        let s_11_0: bool = fn_state.gs_323194;
        // D s_11_1: write-var gs#323195 <= s_11_0
        fn_state.gs_323195 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#323195:u8
        let s_12_0: bool = fn_state.gs_323195;
        // N s_12_1: branch s_12_0 b19 b13
        if s_12_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var imod:u8
        let s_13_0: u8 = fn_state.imod;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 2u16);
        // C s_13_2: const #2u : u8
        let s_13_2: u8 = 2;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 2u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // D s_13_5: write-var enable <= s_13_4
        fn_state.enable = s_13_4;
        // D s_13_6: read-var imod:u8
        let s_13_6: u8 = fn_state.imod;
        // D s_13_7: cast zx s_13_6 -> bv
        let s_13_7: Bits = Bits::new(s_13_6 as u128, 2u16);
        // C s_13_8: const #3u : u8
        let s_13_8: u8 = 3;
        // C s_13_9: cast zx s_13_8 -> bv
        let s_13_9: Bits = Bits::new(s_13_8 as u128, 2u16);
        // D s_13_10: cmp-eq s_13_7 s_13_9
        let s_13_10: bool = ((s_13_7) == (s_13_9));
        // D s_13_11: write-var disable <= s_13_10
        fn_state.disable = s_13_10;
        // D s_13_12: read-var M:u8
        let s_13_12: bool = fn_state.M;
        // D s_13_13: cast zx s_13_12 -> bv
        let s_13_13: Bits = Bits::new(s_13_12 as u128, 1u16);
        // C s_13_14: const #1u : u8
        let s_13_14: bool = true;
        // C s_13_15: cast zx s_13_14 -> bv
        let s_13_15: Bits = Bits::new(s_13_14 as u128, 1u16);
        // D s_13_16: cmp-eq s_13_13 s_13_15
        let s_13_16: bool = ((s_13_13) == (s_13_15));
        // D s_13_17: write-var changemode <= s_13_16
        fn_state.changemode = s_13_16;
        // D s_13_18: read-var A:u8
        let s_13_18: bool = fn_state.A;
        // D s_13_19: cast zx s_13_18 -> bv
        let s_13_19: Bits = Bits::new(s_13_18 as u128, 1u16);
        // C s_13_20: const #1u : u8
        let s_13_20: bool = true;
        // C s_13_21: cast zx s_13_20 -> bv
        let s_13_21: Bits = Bits::new(s_13_20 as u128, 1u16);
        // D s_13_22: cmp-eq s_13_19 s_13_21
        let s_13_22: bool = ((s_13_19) == (s_13_21));
        // D s_13_23: write-var affectA <= s_13_22
        fn_state.affectA = s_13_22;
        // D s_13_24: read-var I:u8
        let s_13_24: bool = fn_state.I;
        // D s_13_25: cast zx s_13_24 -> bv
        let s_13_25: Bits = Bits::new(s_13_24 as u128, 1u16);
        // C s_13_26: const #1u : u8
        let s_13_26: bool = true;
        // C s_13_27: cast zx s_13_26 -> bv
        let s_13_27: Bits = Bits::new(s_13_26 as u128, 1u16);
        // D s_13_28: cmp-eq s_13_25 s_13_27
        let s_13_28: bool = ((s_13_25) == (s_13_27));
        // D s_13_29: write-var affectI <= s_13_28
        fn_state.affectI = s_13_28;
        // D s_13_30: read-var F:u8
        let s_13_30: bool = fn_state.F;
        // D s_13_31: cast zx s_13_30 -> bv
        let s_13_31: Bits = Bits::new(s_13_30 as u128, 1u16);
        // C s_13_32: const #1u : u8
        let s_13_32: bool = true;
        // C s_13_33: cast zx s_13_32 -> bv
        let s_13_33: Bits = Bits::new(s_13_32 as u128, 1u16);
        // D s_13_34: cmp-eq s_13_31 s_13_33
        let s_13_34: bool = ((s_13_31) == (s_13_33));
        // D s_13_35: write-var affectF <= s_13_34
        fn_state.affectF = s_13_34;
        // D s_13_36: read-var imod:u8
        let s_13_36: u8 = fn_state.imod;
        // D s_13_37: cast zx s_13_36 -> bv
        let s_13_37: Bits = Bits::new(s_13_36 as u128, 2u16);
        // C s_13_38: const #1u : u8
        let s_13_38: u8 = 1;
        // C s_13_39: cast zx s_13_38 -> bv
        let s_13_39: Bits = Bits::new(s_13_38 as u128, 2u16);
        // D s_13_40: cmp-eq s_13_37 s_13_39
        let s_13_40: bool = ((s_13_37) == (s_13_39));
        // N s_13_41: branch s_13_40 b18 b14
        if s_13_40 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call InITBlock(s_14_0)
        let s_14_1: bool = InITBlock(state, tracer, s_14_0);
        // D s_14_2: write-var gs#323199 <= s_14_1
        fn_state.gs_323199 = s_14_1;
        // N s_14_3: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#323199:u8
        let s_15_0: bool = fn_state.gs_323199;
        // N s_15_1: branch s_15_0 b17 b16
        if s_15_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var affectA:u8
        let s_16_0: bool = fn_state.affectA;
        // D s_16_1: read-var affectF:u8
        let s_16_1: bool = fn_state.affectF;
        // D s_16_2: read-var affectI:u8
        let s_16_2: bool = fn_state.affectI;
        // D s_16_3: read-var changemode:u8
        let s_16_3: bool = fn_state.changemode;
        // D s_16_4: read-var disable:u8
        let s_16_4: bool = fn_state.disable;
        // D s_16_5: read-var enable:u8
        let s_16_5: bool = fn_state.enable;
        // D s_16_6: read-var mode:u8
        let s_16_6: u8 = fn_state.mode;
        // D s_16_7: call execute_aarch32_instrs_CPS_OpT_AS_txt(s_16_0, s_16_1, s_16_2, s_16_3, s_16_4, s_16_5, s_16_6)
        let s_16_7: () = execute_aarch32_instrs_CPS_OpT_AS_txt(
            state,
            tracer,
            s_16_0,
            s_16_1,
            s_16_2,
            s_16_3,
            s_16_4,
            s_16_5,
            s_16_6,
        );
        // N s_16_8: return
        return;
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
        // D s_18_1: write-var gs#323199 <= s_18_0
        fn_state.gs_323199 = s_18_0;
        // N s_18_2: jump b15
        return block_15(state, tracer, fn_state);
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
        // D s_20_0: read-var A:u8
        let s_20_0: bool = fn_state.A;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 1u16);
        // D s_20_2: read-var I:u8
        let s_20_2: bool = fn_state.I;
        // D s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 1u16);
        // D s_20_4: cast reint s_20_1 -> u128
        let s_20_4: u128 = (s_20_1.value() as u128);
        // D s_20_5: size-of s_20_1
        let s_20_5: u16 = s_20_1.length();
        // D s_20_6: cast reint s_20_3 -> u128
        let s_20_6: u128 = (s_20_3.value() as u128);
        // D s_20_7: size-of s_20_3
        let s_20_7: u16 = s_20_3.length();
        // D s_20_8: lsl s_20_4 s_20_7
        let s_20_8: u128 = s_20_4 << s_20_7;
        // D s_20_9: or s_20_8 s_20_6
        let s_20_9: u128 = ((s_20_8) | (s_20_6));
        // D s_20_10: add s_20_5 s_20_7
        let s_20_10: u16 = (s_20_5 + s_20_7);
        // D s_20_11: create-bits s_20_9 s_20_10
        let s_20_11: Bits = Bits::new(s_20_9, s_20_10);
        // D s_20_12: cast reint s_20_11 -> u8
        let s_20_12: u8 = (s_20_11.value() as u8);
        // D s_20_13: cast zx s_20_12 -> bv
        let s_20_13: Bits = Bits::new(s_20_12 as u128, 2u16);
        // D s_20_14: read-var F:u8
        let s_20_14: bool = fn_state.F;
        // D s_20_15: cast zx s_20_14 -> bv
        let s_20_15: Bits = Bits::new(s_20_14 as u128, 1u16);
        // D s_20_16: cast reint s_20_13 -> u128
        let s_20_16: u128 = (s_20_13.value() as u128);
        // D s_20_17: size-of s_20_13
        let s_20_17: u16 = s_20_13.length();
        // D s_20_18: cast reint s_20_15 -> u128
        let s_20_18: u128 = (s_20_15.value() as u128);
        // D s_20_19: size-of s_20_15
        let s_20_19: u16 = s_20_15.length();
        // D s_20_20: lsl s_20_16 s_20_19
        let s_20_20: u128 = s_20_16 << s_20_19;
        // D s_20_21: or s_20_20 s_20_18
        let s_20_21: u128 = ((s_20_20) | (s_20_18));
        // D s_20_22: add s_20_17 s_20_19
        let s_20_22: u16 = (s_20_17 + s_20_19);
        // D s_20_23: create-bits s_20_21 s_20_22
        let s_20_23: Bits = Bits::new(s_20_21, s_20_22);
        // D s_20_24: cast reint s_20_23 -> u8
        let s_20_24: u8 = (s_20_23.value() as u8);
        // D s_20_25: cast zx s_20_24 -> bv
        let s_20_25: Bits = Bits::new(s_20_24 as u128, 3u16);
        // C s_20_26: const #0u : u8
        let s_20_26: u8 = 0;
        // C s_20_27: cast zx s_20_26 -> bv
        let s_20_27: Bits = Bits::new(s_20_26 as u128, 3u16);
        // D s_20_28: cmp-ne s_20_25 s_20_27
        let s_20_28: bool = ((s_20_25) != (s_20_27));
        // D s_20_29: write-var gs#323194 <= s_20_28
        fn_state.gs_323194 = s_20_28;
        // N s_20_30: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#323195 <= s_21_0
        fn_state.gs_323195 = s_21_0;
        // N s_21_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var A:u8
        let s_22_0: bool = fn_state.A;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 1u16);
        // D s_22_2: read-var I:u8
        let s_22_2: bool = fn_state.I;
        // D s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // D s_22_4: cast reint s_22_1 -> u128
        let s_22_4: u128 = (s_22_1.value() as u128);
        // D s_22_5: size-of s_22_1
        let s_22_5: u16 = s_22_1.length();
        // D s_22_6: cast reint s_22_3 -> u128
        let s_22_6: u128 = (s_22_3.value() as u128);
        // D s_22_7: size-of s_22_3
        let s_22_7: u16 = s_22_3.length();
        // D s_22_8: lsl s_22_4 s_22_7
        let s_22_8: u128 = s_22_4 << s_22_7;
        // D s_22_9: or s_22_8 s_22_6
        let s_22_9: u128 = ((s_22_8) | (s_22_6));
        // D s_22_10: add s_22_5 s_22_7
        let s_22_10: u16 = (s_22_5 + s_22_7);
        // D s_22_11: create-bits s_22_9 s_22_10
        let s_22_11: Bits = Bits::new(s_22_9, s_22_10);
        // D s_22_12: cast reint s_22_11 -> u8
        let s_22_12: u8 = (s_22_11.value() as u8);
        // D s_22_13: cast zx s_22_12 -> bv
        let s_22_13: Bits = Bits::new(s_22_12 as u128, 2u16);
        // D s_22_14: read-var F:u8
        let s_22_14: bool = fn_state.F;
        // D s_22_15: cast zx s_22_14 -> bv
        let s_22_15: Bits = Bits::new(s_22_14 as u128, 1u16);
        // D s_22_16: cast reint s_22_13 -> u128
        let s_22_16: u128 = (s_22_13.value() as u128);
        // D s_22_17: size-of s_22_13
        let s_22_17: u16 = s_22_13.length();
        // D s_22_18: cast reint s_22_15 -> u128
        let s_22_18: u128 = (s_22_15.value() as u128);
        // D s_22_19: size-of s_22_15
        let s_22_19: u16 = s_22_15.length();
        // D s_22_20: lsl s_22_16 s_22_19
        let s_22_20: u128 = s_22_16 << s_22_19;
        // D s_22_21: or s_22_20 s_22_18
        let s_22_21: u128 = ((s_22_20) | (s_22_18));
        // D s_22_22: add s_22_17 s_22_19
        let s_22_22: u16 = (s_22_17 + s_22_19);
        // D s_22_23: create-bits s_22_21 s_22_22
        let s_22_23: Bits = Bits::new(s_22_21, s_22_22);
        // D s_22_24: cast reint s_22_23 -> u8
        let s_22_24: u8 = (s_22_23.value() as u8);
        // D s_22_25: cast zx s_22_24 -> bv
        let s_22_25: Bits = Bits::new(s_22_24 as u128, 3u16);
        // C s_22_26: const #0u : u8
        let s_22_26: u8 = 0;
        // C s_22_27: cast zx s_22_26 -> bv
        let s_22_27: Bits = Bits::new(s_22_26 as u128, 3u16);
        // D s_22_28: cmp-eq s_22_25 s_22_27
        let s_22_28: bool = ((s_22_25) == (s_22_27));
        // D s_22_29: write-var gs#323191 <= s_22_28
        fn_state.gs_323191 = s_22_28;
        // N s_22_30: jump b8
        return block_8(state, tracer, fn_state);
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
        // D s_24_0: read-var M:u8
        let s_24_0: bool = fn_state.M;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 1u16);
        // C s_24_2: const #0u : u8
        let s_24_2: bool = false;
        // C s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // D s_24_4: cmp-eq s_24_1 s_24_3
        let s_24_4: bool = ((s_24_1) == (s_24_3));
        // D s_24_5: write-var gs#323188 <= s_24_4
        fn_state.gs_323188 = s_24_4;
        // N s_24_6: jump b5
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
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var M:u8
        let s_26_0: bool = fn_state.M;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 1u16);
        // C s_26_2: const #0u : u8
        let s_26_2: bool = false;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: write-var gs#323187 <= s_26_4
        fn_state.gs_323187 = s_26_4;
        // N s_26_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
