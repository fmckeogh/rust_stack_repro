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
use u_get_SCTLR_EL3_Type_A::*;
use SCTLR_read__2::*;
use u_get_SCTLR_EL1_Type_A::*;
use Unreachable::*;
use TranslationRegime::*;
use ELUsingAArch32::*;
use u_get_SCTLR_EL2_Type_A::*;
use u_get_SCTLR_Type_A::*;
use u_get_HSCTLR_Type_A::*;
use HSCTLR_read::*;
use common::*;
pub fn AlignmentEnforced<T: Tracer>(state: &mut State, tracer: &T, gs_7422: ()) -> bool {
    #[derive(Default)]
    struct FunctionState {
        regime: u32,
        A: bool,
        gs_7422: (),
    }
    let fn_state = FunctionState {
        gs_7422,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call TranslationRegime(s_0_1)
        let s_0_2: u32 = TranslationRegime(state, tracer, s_0_1);
        // D s_0_3: write-var regime <= s_0_2
        fn_state.regime = s_0_2;
        // C s_0_4: const #0u : u32
        let s_0_4: u32 = 0;
        // D s_0_5: read-var regime:u32
        let s_0_5: u32 = fn_state.regime;
        // D s_0_6: cmp-eq s_0_4 s_0_5
        let s_0_6: bool = ((s_0_4) == (s_0_5));
        // D s_0_7: not s_0_6
        let s_0_7: bool = !s_0_6;
        // N s_0_8: branch s_0_7 b3 b1
        if s_0_7 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #17072u : u32
        let s_1_0: u32 = 17072;
        // D s_1_1: read-reg s_1_0:struct
        let s_1_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: call _get_SCTLR_EL3_Type_A(s_1_1)
        let s_1_2: bool = u_get_SCTLR_EL3_Type_A(state, tracer, s_1_1);
        // D s_1_3: write-var A <= s_1_2
        fn_state.A = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var A:u8
        let s_2_0: bool = fn_state.A;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #1u : u8
        let s_2_2: bool = true;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: return s_2_4
        return s_2_4;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #1u : u32
        let s_3_0: u32 = 1;
        // D s_3_1: read-var regime:u32
        let s_3_1: u32 = fn_state.regime;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: not s_3_2
        let s_3_3: bool = !s_3_2;
        // N s_3_4: branch s_3_3 b5 b4
        if s_3_3 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call SCTLR_read__2(s_4_0)
        let s_4_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_4_0);
        // S s_4_2: call _get_SCTLR_Type_A(s_4_1)
        let s_4_2: bool = u_get_SCTLR_Type_A(state, tracer, s_4_1);
        // D s_4_3: write-var A <= s_4_2
        fn_state.A = s_4_2;
        // N s_4_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #2u : u32
        let s_5_0: u32 = 2;
        // D s_5_1: read-var regime:u32
        let s_5_1: u32 = fn_state.regime;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // D s_5_3: not s_5_2
        let s_5_3: bool = !s_5_2;
        // N s_5_4: branch s_5_3 b10 b6
        if s_5_3 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #432u : u32
        let s_6_0: u32 = 432;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: call ELUsingAArch32(s_6_1)
        let s_6_2: bool = ELUsingAArch32(state, tracer, s_6_1);
        // N s_6_3: branch s_6_2 b9 b7
        if s_6_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #20784u : u32
        let s_7_0: u32 = 20784;
        // D s_7_1: read-reg s_7_0:struct
        let s_7_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call _get_SCTLR_EL2_Type_A(s_7_1)
        let s_7_2: bool = u_get_SCTLR_EL2_Type_A(state, tracer, s_7_1);
        // D s_7_3: write-var A <= s_7_2
        fn_state.A = s_7_2;
        // N s_7_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_8_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call HSCTLR_read(s_9_0)
        let s_9_1: ProductType700c18a878c5601b = HSCTLR_read(state, tracer, s_9_0);
        // S s_9_2: call _get_HSCTLR_Type_A(s_9_1)
        let s_9_2: bool = u_get_HSCTLR_Type_A(state, tracer, s_9_1);
        // D s_9_3: write-var A <= s_9_2
        fn_state.A = s_9_2;
        // N s_9_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #3u : u32
        let s_10_0: u32 = 3;
        // D s_10_1: read-var regime:u32
        let s_10_1: u32 = fn_state.regime;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // D s_10_3: not s_10_2
        let s_10_3: bool = !s_10_2;
        // N s_10_4: branch s_10_3 b12 b11
        if s_10_3 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_11_0: const #20784u : u32
        let s_11_0: u32 = 20784;
        // D s_11_1: read-reg s_11_0:struct
        let s_11_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: call _get_SCTLR_EL2_Type_A(s_11_1)
        let s_11_2: bool = u_get_SCTLR_EL2_Type_A(state, tracer, s_11_1);
        // D s_11_3: write-var A <= s_11_2
        fn_state.A = s_11_2;
        // N s_11_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #4u : u32
        let s_12_0: u32 = 4;
        // D s_12_1: read-var regime:u32
        let s_12_1: u32 = fn_state.regime;
        // D s_12_2: cmp-eq s_12_0 s_12_1
        let s_12_2: bool = ((s_12_0) == (s_12_1));
        // D s_12_3: not s_12_2
        let s_12_3: bool = !s_12_2;
        // N s_12_4: branch s_12_3 b17 b13
        if s_12_3 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #440u : u32
        let s_13_0: u32 = 440;
        // D s_13_1: read-reg s_13_0:u8
        let s_13_1: u8 = {
            let value = state.read_register::<u8>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: call ELUsingAArch32(s_13_1)
        let s_13_2: bool = ELUsingAArch32(state, tracer, s_13_1);
        // N s_13_3: branch s_13_2 b16 b14
        if s_13_2 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #90272u : u32
        let s_14_0: u32 = 90272;
        // D s_14_1: read-reg s_14_0:struct
        let s_14_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: call _get_SCTLR_EL1_Type_A(s_14_1)
        let s_14_2: bool = u_get_SCTLR_EL1_Type_A(state, tracer, s_14_1);
        // D s_14_3: write-var A <= s_14_2
        fn_state.A = s_14_2;
        // N s_14_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_15_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call SCTLR_read__2(s_16_0)
        let s_16_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_16_0);
        // S s_16_2: call _get_SCTLR_Type_A(s_16_1)
        let s_16_2: bool = u_get_SCTLR_Type_A(state, tracer, s_16_1);
        // D s_16_3: write-var A <= s_16_2
        fn_state.A = s_16_2;
        // N s_16_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call Unreachable(s_17_0)
        let s_17_1: () = Unreachable(state, tracer, s_17_0);
        // N s_17_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
