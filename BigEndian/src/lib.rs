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
use UsingAArch32::*;
use u_get_SCTLR_EL2_Type_EE::*;
use SCTLR_read__1::*;
use u_get_SCTLRType_E0E::*;
use HaveNV2Ext::*;
use u_get_SCTLRType_EE::*;
use common::*;
pub fn BigEndian<T: Tracer>(state: &mut State, tracer: &T, acctype: u32) -> bool {
    #[derive(Default)]
    struct FunctionState {
        bigend: bool,
        return_value: bool,
        gs_6801: bool,
        acctype: u32,
    }
    let fn_state = FunctionState {
        acctype,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveNV2Ext(s_0_0)
        let s_0_1: bool = HaveNV2Ext(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b11 b1
        if s_0_1 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#6801 <= s_1_0
        fn_state.gs_6801 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#6801:u8
        let s_2_0: bool = fn_state.gs_6801;
        // N s_2_1: branch s_2_0 b10 b3
        if s_2_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call UsingAArch32(s_3_0)
        let s_3_1: bool = UsingAArch32(state, tracer, s_3_0);
        // N s_3_2: branch s_3_1 b9 b4
        if s_3_1 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #16975u : u32
        let s_4_0: u32 = 16975;
        // D s_4_1: read-reg s_4_0:u8
        let s_4_1: u8 = {
            let value = state.read_register::<u8>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 2u16);
        // C s_4_3: const #448u : u32
        let s_4_3: u32 = 448;
        // D s_4_4: read-reg s_4_3:u8
        let s_4_4: u8 = {
            let value = state.read_register::<u8>(s_4_3 as isize);
            tracer.read_register(s_4_3 as isize, value);
            value
        };
        // D s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 2u16);
        // D s_4_6: cmp-eq s_4_2 s_4_5
        let s_4_6: bool = ((s_4_2) == (s_4_5));
        // N s_4_7: branch s_4_6 b8 b5
        if s_4_6 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call SCTLR_read__1(s_5_0)
        let s_5_1: ProductType5c790c8ef59cc8b2 = SCTLR_read__1(state, tracer, s_5_0);
        // S s_5_2: call _get_SCTLRType_EE(s_5_1)
        let s_5_2: bool = u_get_SCTLRType_EE(state, tracer, s_5_1);
        // S s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // C s_5_4: const #0u : u8
        let s_5_4: bool = false;
        // C s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 1u16);
        // S s_5_6: cmp-ne s_5_3 s_5_5
        let s_5_6: bool = ((s_5_3) != (s_5_5));
        // D s_5_7: write-var bigend <= s_5_6
        fn_state.bigend = s_5_6;
        // N s_5_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var bigend:u8
        let s_6_0: bool = fn_state.bigend;
        // D s_6_1: write-var return_value <= s_6_0
        fn_state.return_value = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var return_value:u8
        let s_7_0: bool = fn_state.return_value;
        // N s_7_1: return s_7_0
        return s_7_0;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call SCTLR_read__1(s_8_0)
        let s_8_1: ProductType5c790c8ef59cc8b2 = SCTLR_read__1(state, tracer, s_8_0);
        // S s_8_2: call _get_SCTLRType_E0E(s_8_1)
        let s_8_2: bool = u_get_SCTLRType_E0E(state, tracer, s_8_1);
        // S s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // C s_8_4: const #0u : u8
        let s_8_4: bool = false;
        // C s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 1u16);
        // S s_8_6: cmp-ne s_8_3 s_8_5
        let s_8_6: bool = ((s_8_3) != (s_8_5));
        // D s_8_7: write-var bigend <= s_8_6
        fn_state.bigend = s_8_6;
        // N s_8_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #16974u : u32
        let s_9_0: u32 = 16974;
        // D s_9_1: read-reg s_9_0:u8
        let s_9_1: bool = {
            let value = state.read_register::<bool>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 1u16);
        // C s_9_3: const #0u : u8
        let s_9_3: bool = false;
        // C s_9_4: cast zx s_9_3 -> bv
        let s_9_4: Bits = Bits::new(s_9_3 as u128, 1u16);
        // D s_9_5: cmp-ne s_9_2 s_9_4
        let s_9_5: bool = ((s_9_2) != (s_9_4));
        // D s_9_6: write-var bigend <= s_9_5
        fn_state.bigend = s_9_5;
        // N s_9_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #20784u : u32
        let s_10_0: u32 = 20784;
        // D s_10_1: read-reg s_10_0:struct
        let s_10_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: call _get_SCTLR_EL2_Type_EE(s_10_1)
        let s_10_2: bool = u_get_SCTLR_EL2_Type_EE(state, tracer, s_10_1);
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // C s_10_4: const #1u : u8
        let s_10_4: bool = true;
        // C s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 1u16);
        // D s_10_6: cmp-eq s_10_3 s_10_5
        let s_10_6: bool = ((s_10_3) == (s_10_5));
        // D s_10_7: write-var return_value <= s_10_6
        fn_state.return_value = s_10_6;
        // N s_10_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var acctype:u32
        let s_11_0: u32 = fn_state.acctype;
        // C s_11_1: const #9u : u32
        let s_11_1: u32 = 9;
        // D s_11_2: cmp-eq s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) == (s_11_1));
        // D s_11_3: write-var gs#6801 <= s_11_2
        fn_state.gs_6801 = s_11_2;
        // N s_11_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
