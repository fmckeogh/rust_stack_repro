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
use execute_aarch32_instrs_CPS_OpT_AS_txt::*;
use common::*;
pub fn decode_aarch32_instrs_CPS_A1enc_AS_txt<T: Tracer>(
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
        affectI: bool,
        gs_323178: bool,
        gs_323172: bool,
        enable: bool,
        gs_323175: bool,
        changemode: bool,
        disable: bool,
        gs_323179: bool,
        gs_323183: bool,
        gs_323182: bool,
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
        // D s_0_0: read-var mode:u8
        let s_0_0: u8 = fn_state.mode;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // C s_0_2: const #0u : u8
        let s_0_2: u8 = 0;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 5u16);
        // D s_0_4: cmp-ne s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) != (s_0_3));
        // N s_0_5: branch s_0_4 b24 b1
        if s_0_4 {
            return block_24(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#323172 <= s_1_0
        fn_state.gs_323172 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#323172:u8
        let s_2_0: bool = fn_state.gs_323172;
        // N s_2_1: branch s_2_0 b23 b3
        if s_2_0 {
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
        // C s_3_0: const #1s : i
        let s_3_0: i128 = 1;
        // D s_3_1: read-var imod:u8
        let s_3_1: u8 = fn_state.imod;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 2u16);
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
        // N s_3_22: branch s_3_21 b22 b4
        if s_3_21 {
            return block_22(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#323175 <= s_4_0
        fn_state.gs_323175 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#323175:u8
        let s_5_0: bool = fn_state.gs_323175;
        // N s_5_1: branch s_5_0 b21 b6
        if s_5_0 {
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
        // C s_6_19: const #0u : u8
        let s_6_19: bool = false;
        // C s_6_20: cast zx s_6_19 -> bv
        let s_6_20: Bits = Bits::new(s_6_19 as u128, 1u16);
        // D s_6_21: cmp-eq s_6_18 s_6_20
        let s_6_21: bool = ((s_6_18) == (s_6_20));
        // N s_6_22: branch s_6_21 b20 b7
        if s_6_21 {
            return block_20(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#323178 <= s_7_0
        fn_state.gs_323178 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#323178:u8
        let s_8_0: bool = fn_state.gs_323178;
        // D s_8_1: write-var gs#323179 <= s_8_0
        fn_state.gs_323179 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#323179:u8
        let s_9_0: bool = fn_state.gs_323179;
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
        // D s_10_0: read-var imod:u8
        let s_10_0: u8 = fn_state.imod;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 2u16);
        // C s_10_2: const #2u : u8
        let s_10_2: u8 = 2;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 2u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: write-var enable <= s_10_4
        fn_state.enable = s_10_4;
        // D s_10_6: read-var imod:u8
        let s_10_6: u8 = fn_state.imod;
        // D s_10_7: cast zx s_10_6 -> bv
        let s_10_7: Bits = Bits::new(s_10_6 as u128, 2u16);
        // C s_10_8: const #3u : u8
        let s_10_8: u8 = 3;
        // C s_10_9: cast zx s_10_8 -> bv
        let s_10_9: Bits = Bits::new(s_10_8 as u128, 2u16);
        // D s_10_10: cmp-eq s_10_7 s_10_9
        let s_10_10: bool = ((s_10_7) == (s_10_9));
        // D s_10_11: write-var disable <= s_10_10
        fn_state.disable = s_10_10;
        // D s_10_12: read-var M:u8
        let s_10_12: bool = fn_state.M;
        // D s_10_13: cast zx s_10_12 -> bv
        let s_10_13: Bits = Bits::new(s_10_12 as u128, 1u16);
        // C s_10_14: const #1u : u8
        let s_10_14: bool = true;
        // C s_10_15: cast zx s_10_14 -> bv
        let s_10_15: Bits = Bits::new(s_10_14 as u128, 1u16);
        // D s_10_16: cmp-eq s_10_13 s_10_15
        let s_10_16: bool = ((s_10_13) == (s_10_15));
        // D s_10_17: write-var changemode <= s_10_16
        fn_state.changemode = s_10_16;
        // D s_10_18: read-var A:u8
        let s_10_18: bool = fn_state.A;
        // D s_10_19: cast zx s_10_18 -> bv
        let s_10_19: Bits = Bits::new(s_10_18 as u128, 1u16);
        // C s_10_20: const #1u : u8
        let s_10_20: bool = true;
        // C s_10_21: cast zx s_10_20 -> bv
        let s_10_21: Bits = Bits::new(s_10_20 as u128, 1u16);
        // D s_10_22: cmp-eq s_10_19 s_10_21
        let s_10_22: bool = ((s_10_19) == (s_10_21));
        // D s_10_23: write-var affectA <= s_10_22
        fn_state.affectA = s_10_22;
        // D s_10_24: read-var I:u8
        let s_10_24: bool = fn_state.I;
        // D s_10_25: cast zx s_10_24 -> bv
        let s_10_25: Bits = Bits::new(s_10_24 as u128, 1u16);
        // C s_10_26: const #1u : u8
        let s_10_26: bool = true;
        // C s_10_27: cast zx s_10_26 -> bv
        let s_10_27: Bits = Bits::new(s_10_26 as u128, 1u16);
        // D s_10_28: cmp-eq s_10_25 s_10_27
        let s_10_28: bool = ((s_10_25) == (s_10_27));
        // D s_10_29: write-var affectI <= s_10_28
        fn_state.affectI = s_10_28;
        // D s_10_30: read-var F:u8
        let s_10_30: bool = fn_state.F;
        // D s_10_31: cast zx s_10_30 -> bv
        let s_10_31: Bits = Bits::new(s_10_30 as u128, 1u16);
        // C s_10_32: const #1u : u8
        let s_10_32: bool = true;
        // C s_10_33: cast zx s_10_32 -> bv
        let s_10_33: Bits = Bits::new(s_10_32 as u128, 1u16);
        // D s_10_34: cmp-eq s_10_31 s_10_33
        let s_10_34: bool = ((s_10_31) == (s_10_33));
        // D s_10_35: write-var affectF <= s_10_34
        fn_state.affectF = s_10_34;
        // D s_10_36: read-var imod:u8
        let s_10_36: u8 = fn_state.imod;
        // D s_10_37: cast zx s_10_36 -> bv
        let s_10_37: Bits = Bits::new(s_10_36 as u128, 2u16);
        // C s_10_38: const #0u : u8
        let s_10_38: u8 = 0;
        // C s_10_39: cast zx s_10_38 -> bv
        let s_10_39: Bits = Bits::new(s_10_38 as u128, 2u16);
        // D s_10_40: cmp-eq s_10_37 s_10_39
        let s_10_40: bool = ((s_10_37) == (s_10_39));
        // N s_10_41: branch s_10_40 b18 b11
        if s_10_40 {
            return block_18(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#323182 <= s_11_0
        fn_state.gs_323182 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#323182:u8
        let s_12_0: bool = fn_state.gs_323182;
        // N s_12_1: branch s_12_0 b17 b13
        if s_12_0 {
            return block_17(state, tracer, fn_state);
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
        // C s_13_2: const #1u : u8
        let s_13_2: u8 = 1;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 2u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // D s_13_5: write-var gs#323183 <= s_13_4
        fn_state.gs_323183 = s_13_4;
        // N s_13_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#323183:u8
        let s_14_0: bool = fn_state.gs_323183;
        // N s_14_1: branch s_14_0 b16 b15
        if s_14_0 {
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
        // D s_15_0: read-var affectA:u8
        let s_15_0: bool = fn_state.affectA;
        // D s_15_1: read-var affectF:u8
        let s_15_1: bool = fn_state.affectF;
        // D s_15_2: read-var affectI:u8
        let s_15_2: bool = fn_state.affectI;
        // D s_15_3: read-var changemode:u8
        let s_15_3: bool = fn_state.changemode;
        // D s_15_4: read-var disable:u8
        let s_15_4: bool = fn_state.disable;
        // D s_15_5: read-var enable:u8
        let s_15_5: bool = fn_state.enable;
        // D s_15_6: read-var mode:u8
        let s_15_6: u8 = fn_state.mode;
        // D s_15_7: call execute_aarch32_instrs_CPS_OpT_AS_txt(s_15_0, s_15_1, s_15_2, s_15_3, s_15_4, s_15_5, s_15_6)
        let s_15_7: () = execute_aarch32_instrs_CPS_OpT_AS_txt(
            state,
            tracer,
            s_15_0,
            s_15_1,
            s_15_2,
            s_15_3,
            s_15_4,
            s_15_5,
            s_15_6,
        );
        // N s_15_8: return
        return;
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
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#323183 <= s_17_0
        fn_state.gs_323183 = s_17_0;
        // N s_17_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var M:u8
        let s_18_0: bool = fn_state.M;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 1u16);
        // C s_18_2: const #0u : u8
        let s_18_2: bool = false;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // D s_18_5: write-var gs#323182 <= s_18_4
        fn_state.gs_323182 = s_18_4;
        // N s_18_6: jump b12
        return block_12(state, tracer, fn_state);
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
        // D s_20_29: write-var gs#323178 <= s_20_28
        fn_state.gs_323178 = s_20_28;
        // N s_20_30: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#323179 <= s_21_0
        fn_state.gs_323179 = s_21_0;
        // N s_21_2: jump b9
        return block_9(state, tracer, fn_state);
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
        // D s_22_29: write-var gs#323175 <= s_22_28
        fn_state.gs_323175 = s_22_28;
        // N s_22_30: jump b5
        return block_5(state, tracer, fn_state);
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
        // D s_24_5: write-var gs#323172 <= s_24_4
        fn_state.gs_323172 = s_24_4;
        // N s_24_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
