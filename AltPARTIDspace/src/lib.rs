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
use Unreachable::*;
use u_get_MPAM3_EL3_Type_ALTSP_EL3::*;
use AltPIdRealm::*;
use u_get_MPAM3_EL3_Type_RT_ALTSP_NS::*;
use MPAM3_EL3_read::*;
use AltPIdSecure::*;
use common::*;
pub fn AltPARTIDspace<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    security: u32,
    primaryPIdSpace: u32,
) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        ga_4763: u32,
        return_value: u32,
        el: u8,
        security: u32,
        primaryPIdSpace: u32,
    }
    let fn_state = FunctionState {
        el,
        security,
        primaryPIdSpace,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_0_0: const #0u : u32
        let s_0_0: u32 = 0;
        // D s_0_1: read-var security:u32
        let s_0_1: u32 = fn_state.security;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // D s_0_3: not s_0_2
        let s_0_3: bool = !s_0_2;
        // N s_0_4: branch s_0_3 b3 b1
        if s_0_3 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_1_0: read-var el:u8
        let s_1_0: u8 = fn_state.el;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // C s_1_2: const #424u : u32
        let s_1_2: u32 = 424;
        // D s_1_3: read-reg s_1_2:u8
        let s_1_3: u8 = {
            let value = state.read_register::<u8>(s_1_2 as isize);
            tracer.read_register(s_1_2 as isize, value);
            value
        };
        // D s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 2u16);
        // D s_1_5: cmp-ne s_1_1 s_1_4
        let s_1_5: bool = ((s_1_1) != (s_1_4));
        // N s_1_6: assert s_1_5
        let s_1_6: () = assert!(s_1_5);
        // D s_1_7: read-var primaryPIdSpace:u32
        let s_1_7: u32 = fn_state.primaryPIdSpace;
        // D s_1_8: write-var return_value <= s_1_7
        fn_state.return_value = s_1_7;
        // N s_1_9: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_2_0: read-var return_value:u32
        let s_2_0: u32 = fn_state.return_value;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_3_0: const #3u : u32
        let s_3_0: u32 = 3;
        // D s_3_1: read-var security:u32
        let s_3_1: u32 = fn_state.security;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: not s_3_2
        let s_3_3: bool = !s_3_2;
        // N s_3_4: branch s_3_3 b7 b4
        if s_3_3 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_4_0: read-var el:u8
        let s_4_0: u8 = fn_state.el;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #424u : u32
        let s_4_2: u32 = 424;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 2u16);
        // D s_4_5: cmp-ne s_4_1 s_4_4
        let s_4_5: bool = ((s_4_1) != (s_4_4));
        // N s_4_6: assert s_4_5
        let s_4_6: () = assert!(s_4_5);
        // D s_4_7: read-var primaryPIdSpace:u32
        let s_4_7: u32 = fn_state.primaryPIdSpace;
        // C s_4_8: const #3u : u32
        let s_4_8: u32 = 3;
        // D s_4_9: cmp-eq s_4_7 s_4_8
        let s_4_9: bool = ((s_4_7) == (s_4_8));
        // N s_4_10: branch s_4_9 b6 b5
        if s_4_9 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_5_0: read-var el:u8
        let s_5_0: u8 = fn_state.el;
        // D s_5_1: read-var primaryPIdSpace:u32
        let s_5_1: u32 = fn_state.primaryPIdSpace;
        // D s_5_2: call AltPIdSecure(s_5_0, s_5_1)
        let s_5_2: u32 = AltPIdSecure(state, tracer, s_5_0, s_5_1);
        // D s_5_3: write-var return_value <= s_5_2
        fn_state.return_value = s_5_2;
        // N s_5_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_6_0: read-var primaryPIdSpace:u32
        let s_6_0: u32 = fn_state.primaryPIdSpace;
        // D s_6_1: write-var return_value <= s_6_0
        fn_state.return_value = s_6_0;
        // N s_6_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_7_0: const #1u : u32
        let s_7_0: u32 = 1;
        // D s_7_1: read-var security:u32
        let s_7_1: u32 = fn_state.security;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // D s_7_3: not s_7_2
        let s_7_3: bool = !s_7_2;
        // N s_7_4: branch s_7_3 b13 b8
        if s_7_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_8_0: read-var el:u8
        let s_8_0: u8 = fn_state.el;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 2u16);
        // C s_8_2: const #424u : u32
        let s_8_2: u32 = 424;
        // D s_8_3: read-reg s_8_2:u8
        let s_8_3: u8 = {
            let value = state.read_register::<u8>(s_8_2 as isize);
            tracer.read_register(s_8_2 as isize, value);
            value
        };
        // D s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 2u16);
        // D s_8_5: cmp-eq s_8_1 s_8_4
        let s_8_5: bool = ((s_8_1) == (s_8_4));
        // N s_8_6: assert s_8_5
        let s_8_6: () = assert!(s_8_5);
        // C s_8_7: const #() : ()
        let s_8_7: () = ();
        // S s_8_8: call MPAM3_EL3_read(s_8_7)
        let s_8_8: ProductType5c790c8ef59cc8b2 = MPAM3_EL3_read(state, tracer, s_8_7);
        // S s_8_9: call _get_MPAM3_EL3_Type_ALTSP_EL3(s_8_8)
        let s_8_9: bool = u_get_MPAM3_EL3_Type_ALTSP_EL3(state, tracer, s_8_8);
        // S s_8_10: cast zx s_8_9 -> bv
        let s_8_10: Bits = Bits::new(s_8_9 as u128, 1u16);
        // C s_8_11: const #1u : u8
        let s_8_11: bool = true;
        // C s_8_12: cast zx s_8_11 -> bv
        let s_8_12: Bits = Bits::new(s_8_11 as u128, 1u16);
        // S s_8_13: cmp-eq s_8_10 s_8_12
        let s_8_13: bool = ((s_8_10) == (s_8_12));
        // N s_8_14: branch s_8_13 b10 b9
        if s_8_13 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_9_0: read-var primaryPIdSpace:u32
        let s_9_0: u32 = fn_state.primaryPIdSpace;
        // D s_9_1: write-var return_value <= s_9_0
        fn_state.return_value = s_9_0;
        // N s_9_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call MPAM3_EL3_read(s_10_0)
        let s_10_1: ProductType5c790c8ef59cc8b2 = MPAM3_EL3_read(state, tracer, s_10_0);
        // S s_10_2: call _get_MPAM3_EL3_Type_RT_ALTSP_NS(s_10_1)
        let s_10_2: bool = u_get_MPAM3_EL3_Type_RT_ALTSP_NS(state, tracer, s_10_1);
        // S s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // C s_10_4: const #1u : u8
        let s_10_4: bool = true;
        // C s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 1u16);
        // S s_10_6: cmp-eq s_10_3 s_10_5
        let s_10_6: bool = ((s_10_3) == (s_10_5));
        // N s_10_7: branch s_10_6 b12 b11
        if s_10_6 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_11_0: const #0u : u32
        let s_11_0: u32 = 0;
        // D s_11_1: write-var return_value <= s_11_0
        fn_state.return_value = s_11_0;
        // N s_11_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_12_0: const #3u : u32
        let s_12_0: u32 = 3;
        // D s_12_1: write-var return_value <= s_12_0
        fn_state.return_value = s_12_0;
        // N s_12_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_13_0: const #2u : u32
        let s_13_0: u32 = 2;
        // D s_13_1: read-var security:u32
        let s_13_1: u32 = fn_state.security;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // D s_13_3: not s_13_2
        let s_13_3: bool = !s_13_2;
        // N s_13_4: branch s_13_3 b15 b14
        if s_13_3 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_14_0: read-var el:u8
        let s_14_0: u8 = fn_state.el;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 2u16);
        // C s_14_2: const #424u : u32
        let s_14_2: u32 = 424;
        // D s_14_3: read-reg s_14_2:u8
        let s_14_3: u8 = {
            let value = state.read_register::<u8>(s_14_2 as isize);
            tracer.read_register(s_14_2 as isize, value);
            value
        };
        // D s_14_4: cast zx s_14_3 -> bv
        let s_14_4: Bits = Bits::new(s_14_3 as u128, 2u16);
        // D s_14_5: cmp-ne s_14_1 s_14_4
        let s_14_5: bool = ((s_14_1) != (s_14_4));
        // N s_14_6: assert s_14_5
        let s_14_6: () = assert!(s_14_5);
        // D s_14_7: read-var el:u8
        let s_14_7: u8 = fn_state.el;
        // D s_14_8: read-var primaryPIdSpace:u32
        let s_14_8: u32 = fn_state.primaryPIdSpace;
        // D s_14_9: call AltPIdRealm(s_14_7, s_14_8)
        let s_14_9: u32 = AltPIdRealm(state, tracer, s_14_7, s_14_8);
        // D s_14_10: write-var return_value <= s_14_9
        fn_state.return_value = s_14_9;
        // N s_14_11: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call Unreachable(s_15_0)
        let s_15_1: () = Unreachable(state, tracer, s_15_0);
        // D s_15_2: read-var ga#4763:u32
        let s_15_2: u32 = fn_state.ga_4763;
        // D s_15_3: write-var return_value <= s_15_2
        fn_state.return_value = s_15_2;
        // N s_15_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
