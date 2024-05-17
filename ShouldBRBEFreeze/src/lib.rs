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
use BranchRecordAllowed::*;
use u_get_BRBCR_EL2_Type_FZP::*;
use PMUOverflowCondition::*;
use u_get_BRBCR_EL1_Type_FZP::*;
use common::*;
pub fn ShouldBRBEFreeze<T: Tracer>(state: &mut State, tracer: &T, gs_4688: ()) -> bool {
    #[derive(Default)]
    struct FunctionState {
        return_value: bool,
        include_hi_name: bool,
        include_lo_name: bool,
        gs_4688: (),
    }
    let fn_state = FunctionState {
        gs_4688,
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
        // D s_0_2: call BranchRecordAllowed(s_0_1)
        let s_0_2: bool = BranchRecordAllowed(state, tracer, s_0_1);
        // D s_0_3: not s_0_2
        let s_0_3: bool = !s_0_2;
        // N s_0_4: branch s_0_3 b6 b1
        if s_0_3 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #432u : u32
        let s_1_0: u32 = 432;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // C s_1_2: const #2u : u8
        let s_1_2: u8 = 2;
        // D s_1_3: cmp-lt s_1_1 s_1_2
        let s_1_3: bool = ((s_1_1) < (s_1_2));
        // N s_1_4: branch s_1_3 b5 b2
        if s_1_3 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #1u : u8
        let s_2_0: bool = true;
        // D s_2_1: write-var include_lo_name <= s_2_0
        fn_state.include_lo_name = s_2_0;
        // C s_2_2: const #1u : u8
        let s_2_2: bool = true;
        // D s_2_3: write-var include_hi_name <= s_2_2
        fn_state.include_hi_name = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // C s_3_1: const #0u : u8
        let s_3_1: bool = false;
        // C s_3_2: const #0u : u8
        let s_3_2: bool = false;
        // D s_3_3: read-var include_hi_name:u8
        let s_3_3: bool = fn_state.include_hi_name;
        // D s_3_4: read-var include_lo_name:u8
        let s_3_4: bool = fn_state.include_lo_name;
        // C s_3_5: const #1u : u8
        let s_3_5: bool = true;
        // C s_3_6: const #0u : u8
        let s_3_6: bool = false;
        // D s_3_7: call PMUOverflowCondition(s_3_0, s_3_1, s_3_2, s_3_3, s_3_4, s_3_5, s_3_6)
        let s_3_7: bool = PMUOverflowCondition(
            state,
            tracer,
            s_3_0,
            s_3_1,
            s_3_2,
            s_3_3,
            s_3_4,
            s_3_5,
            s_3_6,
        );
        // D s_3_8: write-var return_value <= s_3_7
        fn_state.return_value = s_3_7;
        // N s_3_9: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var return_value:u8
        let s_4_0: bool = fn_state.return_value;
        // N s_4_1: return s_4_0
        return s_4_0;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #90640u : u32
        let s_5_0: u32 = 90640;
        // D s_5_1: read-reg s_5_0:struct
        let s_5_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call _get_BRBCR_EL1_Type_FZP(s_5_1)
        let s_5_2: bool = u_get_BRBCR_EL1_Type_FZP(state, tracer, s_5_1);
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // C s_5_4: const #1u : u8
        let s_5_4: bool = true;
        // C s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 1u16);
        // D s_5_6: cmp-eq s_5_3 s_5_5
        let s_5_6: bool = ((s_5_3) == (s_5_5));
        // D s_5_7: write-var include_lo_name <= s_5_6
        fn_state.include_lo_name = s_5_6;
        // C s_5_8: const #18272u : u32
        let s_5_8: u32 = 18272;
        // D s_5_9: read-reg s_5_8:struct
        let s_5_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_8 as isize);
            tracer.read_register(s_5_8 as isize, value);
            value
        };
        // D s_5_10: call _get_BRBCR_EL2_Type_FZP(s_5_9)
        let s_5_10: bool = u_get_BRBCR_EL2_Type_FZP(state, tracer, s_5_9);
        // D s_5_11: cast zx s_5_10 -> bv
        let s_5_11: Bits = Bits::new(s_5_10 as u128, 1u16);
        // C s_5_12: const #1u : u8
        let s_5_12: bool = true;
        // C s_5_13: cast zx s_5_12 -> bv
        let s_5_13: Bits = Bits::new(s_5_12 as u128, 1u16);
        // D s_5_14: cmp-eq s_5_11 s_5_13
        let s_5_14: bool = ((s_5_11) == (s_5_13));
        // D s_5_15: write-var include_hi_name <= s_5_14
        fn_state.include_hi_name = s_5_14;
        // N s_5_16: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var return_value <= s_6_0
        fn_state.return_value = s_6_0;
        // N s_6_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
