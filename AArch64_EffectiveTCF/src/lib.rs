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
use HaveMTEAsymFaultExt::*;
use u_get_SCTLR_EL2_Type_TCF0::*;
use TranslationRegime::*;
use u_get_SCTLR_EL3_Type_TCF::*;
use ConstrainUnpredictableBits::*;
use u_get_SCTLR_EL2_Type_TCF::*;
use u_get_SCTLR_EL1_Type_TCF0::*;
use u_get_SCTLR_EL1_Type_TCF::*;
use Unreachable::*;
use common::*;
pub fn AArch64_EffectiveTCF<T: Tracer>(state: &mut State, tracer: &T, el: u8) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        tcf: u8,
        regime: u32,
        gs_444124: ProductType690b94b58c91cec7,
        el: u8,
    }
    let fn_state = FunctionState {
        el,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_0_0: read-var el:u8
        let s_0_0: u8 = fn_state.el;
        // D s_0_1: call TranslationRegime(s_0_0)
        let s_0_1: u32 = TranslationRegime(state, tracer, s_0_0);
        // D s_0_2: write-var regime <= s_0_1
        fn_state.regime = s_0_1;
        // C s_0_3: const #0u : u32
        let s_0_3: u32 = 0;
        // D s_0_4: read-var regime:u32
        let s_0_4: u32 = fn_state.regime;
        // D s_0_5: cmp-eq s_0_3 s_0_4
        let s_0_5: bool = ((s_0_3) == (s_0_4));
        // D s_0_6: not s_0_5
        let s_0_6: bool = !s_0_5;
        // N s_0_7: branch s_0_6 b9 b1
        if s_0_6 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_1_0: const #17072u : u32
        let s_1_0: u32 = 17072;
        // D s_1_1: read-reg s_1_0:struct
        let s_1_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: call _get_SCTLR_EL3_Type_TCF(s_1_1)
        let s_1_2: u8 = u_get_SCTLR_EL3_Type_TCF(state, tracer, s_1_1);
        // D s_1_3: write-var tcf <= s_1_2
        fn_state.tcf = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_2_0: read-var tcf:u8
        let s_2_0: u8 = fn_state.tcf;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #3u : u8
        let s_2_2: u8 = 3;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b5 b3
        if s_2_4 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_4_0: read-var tcf:u8
        let s_4_0: u8 = fn_state.tcf;
        // N s_4_1: return s_4_0
        return s_4_0;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call HaveMTEAsymFaultExt(s_5_0)
        let s_5_1: bool = HaveMTEAsymFaultExt(state, tracer, s_5_0);
        // S s_5_2: not s_5_1
        let s_5_2: bool = !s_5_1;
        // N s_5_3: branch s_5_2 b8 b6
        if s_5_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // N s_7_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_8_0: const #2s : i
        let s_8_0: i128 = 2;
        // C s_8_1: const #17u : u32
        let s_8_1: u32 = 17;
        // S s_8_2: call ConstrainUnpredictableBits(s_8_1, s_8_0)
        let s_8_2: ProductType690b94b58c91cec7 = ConstrainUnpredictableBits(
            state,
            tracer,
            s_8_1,
            s_8_0,
        );
        // D s_8_3: write-var gs#444124 <= s_8_2
        fn_state.gs_444124 = s_8_2;
        // D s_8_4: read-var gs#444124.1:struct
        let s_8_4: Bits = fn_state.gs_444124._1;
        // D s_8_5: cast reint s_8_4 -> u8
        let s_8_5: u8 = (s_8_4.value() as u8);
        // D s_8_6: write-var tcf <= s_8_5
        fn_state.tcf = s_8_5;
        // N s_8_7: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_9_0: const #2u : u32
        let s_9_0: u32 = 2;
        // D s_9_1: read-var regime:u32
        let s_9_1: u32 = fn_state.regime;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // D s_9_3: not s_9_2
        let s_9_3: bool = !s_9_2;
        // N s_9_4: branch s_9_3 b11 b10
        if s_9_3 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_10_0: const #20784u : u32
        let s_10_0: u32 = 20784;
        // D s_10_1: read-reg s_10_0:struct
        let s_10_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: call _get_SCTLR_EL2_Type_TCF(s_10_1)
        let s_10_2: u8 = u_get_SCTLR_EL2_Type_TCF(state, tracer, s_10_1);
        // D s_10_3: write-var tcf <= s_10_2
        fn_state.tcf = s_10_2;
        // N s_10_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_11_0: const #3u : u32
        let s_11_0: u32 = 3;
        // D s_11_1: read-var regime:u32
        let s_11_1: u32 = fn_state.regime;
        // D s_11_2: cmp-eq s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) == (s_11_1));
        // D s_11_3: not s_11_2
        let s_11_3: bool = !s_11_2;
        // N s_11_4: branch s_11_3 b16 b12
        if s_11_3 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_12_0: read-var el:u8
        let s_12_0: u8 = fn_state.el;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 2u16);
        // C s_12_2: const #448u : u32
        let s_12_2: u32 = 448;
        // D s_12_3: read-reg s_12_2:u8
        let s_12_3: u8 = {
            let value = state.read_register::<u8>(s_12_2 as isize);
            tracer.read_register(s_12_2 as isize, value);
            value
        };
        // D s_12_4: cast zx s_12_3 -> bv
        let s_12_4: Bits = Bits::new(s_12_3 as u128, 2u16);
        // D s_12_5: cmp-eq s_12_1 s_12_4
        let s_12_5: bool = ((s_12_1) == (s_12_4));
        // N s_12_6: branch s_12_5 b15 b13
        if s_12_5 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_13_0: const #20784u : u32
        let s_13_0: u32 = 20784;
        // D s_13_1: read-reg s_13_0:struct
        let s_13_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: call _get_SCTLR_EL2_Type_TCF(s_13_1)
        let s_13_2: u8 = u_get_SCTLR_EL2_Type_TCF(state, tracer, s_13_1);
        // D s_13_3: write-var tcf <= s_13_2
        fn_state.tcf = s_13_2;
        // N s_13_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // N s_14_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_15_0: const #20784u : u32
        let s_15_0: u32 = 20784;
        // D s_15_1: read-reg s_15_0:struct
        let s_15_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // D s_15_2: call _get_SCTLR_EL2_Type_TCF0(s_15_1)
        let s_15_2: u8 = u_get_SCTLR_EL2_Type_TCF0(state, tracer, s_15_1);
        // D s_15_3: write-var tcf <= s_15_2
        fn_state.tcf = s_15_2;
        // N s_15_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_16_0: const #4u : u32
        let s_16_0: u32 = 4;
        // D s_16_1: read-var regime:u32
        let s_16_1: u32 = fn_state.regime;
        // D s_16_2: cmp-eq s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) == (s_16_1));
        // D s_16_3: not s_16_2
        let s_16_3: bool = !s_16_2;
        // N s_16_4: branch s_16_3 b21 b17
        if s_16_3 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_17_0: read-var el:u8
        let s_17_0: u8 = fn_state.el;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 2u16);
        // C s_17_2: const #448u : u32
        let s_17_2: u32 = 448;
        // D s_17_3: read-reg s_17_2:u8
        let s_17_3: u8 = {
            let value = state.read_register::<u8>(s_17_2 as isize);
            tracer.read_register(s_17_2 as isize, value);
            value
        };
        // D s_17_4: cast zx s_17_3 -> bv
        let s_17_4: Bits = Bits::new(s_17_3 as u128, 2u16);
        // D s_17_5: cmp-eq s_17_1 s_17_4
        let s_17_5: bool = ((s_17_1) == (s_17_4));
        // N s_17_6: branch s_17_5 b20 b18
        if s_17_5 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_18_0: const #90272u : u32
        let s_18_0: u32 = 90272;
        // D s_18_1: read-reg s_18_0:struct
        let s_18_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: call _get_SCTLR_EL1_Type_TCF(s_18_1)
        let s_18_2: u8 = u_get_SCTLR_EL1_Type_TCF(state, tracer, s_18_1);
        // D s_18_3: write-var tcf <= s_18_2
        fn_state.tcf = s_18_2;
        // N s_18_4: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // N s_19_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_20_0: const #90272u : u32
        let s_20_0: u32 = 90272;
        // D s_20_1: read-reg s_20_0:struct
        let s_20_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // D s_20_2: call _get_SCTLR_EL1_Type_TCF0(s_20_1)
        let s_20_2: u8 = u_get_SCTLR_EL1_Type_TCF0(state, tracer, s_20_1);
        // D s_20_3: write-var tcf <= s_20_2
        fn_state.tcf = s_20_2;
        // N s_20_4: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call Unreachable(s_21_0)
        let s_21_1: () = Unreachable(state, tracer, s_21_0);
        // N s_21_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
