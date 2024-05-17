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
use u_get_BRBTGTINJ_EL1_Type_ADDRESS::*;
use u_get_BRBINFINJ_EL1_Type_VALID::*;
use common::*;
pub fn u__set_BRBTGTINJ_EL1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType5c790c8ef59cc8b2,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType5c790c8ef59cc8b2,
        gs_34022: bool,
        value_name: ProductType5c790c8ef59cc8b2,
    }
    let fn_state = FunctionState {
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType5c790c8ef59cc8b2 = fn_state.value_name;
        // D s_0_1: write-var r <= s_0_0
        fn_state.r = s_0_0;
        // C s_0_2: const #10168u : u32
        let s_0_2: u32 = 10168;
        // D s_0_3: read-reg s_0_2:struct
        let s_0_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // D s_0_4: call _get_BRBINFINJ_EL1_Type_VALID(s_0_3)
        let s_0_4: u8 = u_get_BRBINFINJ_EL1_Type_VALID(state, tracer, s_0_3);
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // C s_0_6: const #0u : u8
        let s_0_6: u8 = 0;
        // C s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 2u16);
        // D s_0_8: cmp-eq s_0_5 s_0_7
        let s_0_8: bool = ((s_0_5) == (s_0_7));
        // N s_0_9: branch s_0_8 b5 b1
        if s_0_8 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #10168u : u32
        let s_1_0: u32 = 10168;
        // D s_1_1: read-reg s_1_0:struct
        let s_1_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: call _get_BRBINFINJ_EL1_Type_VALID(s_1_1)
        let s_1_2: u8 = u_get_BRBINFINJ_EL1_Type_VALID(state, tracer, s_1_1);
        // D s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 2u16);
        // C s_1_4: const #2u : u8
        let s_1_4: u8 = 2;
        // C s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-eq s_1_3 s_1_5
        let s_1_6: bool = ((s_1_3) == (s_1_5));
        // D s_1_7: write-var gs#34022 <= s_1_6
        fn_state.gs_34022 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#34022:u8
        let s_2_0: bool = fn_state.gs_34022;
        // D s_2_1: not s_2_0
        let s_2_1: bool = !s_2_0;
        // N s_2_2: branch s_2_1 b4 b3
        if s_2_1 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var r:struct
        let s_4_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_4_1: call _get_BRBTGTINJ_EL1_Type_ADDRESS(s_4_0)
        let s_4_1: u64 = u_get_BRBTGTINJ_EL1_Type_ADDRESS(state, tracer, s_4_0);
        // C s_4_2: const #102304u : u32
        let s_4_2: u32 = 102304;
        // D s_4_3: read-reg s_4_2:struct
        let s_4_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // C s_4_4: const #102304u : u32
        let s_4_4: u32 = 102304;
        // N s_4_5: write-reg s_4_4 <= s_4_3
        let s_4_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_4_4 as isize, s_4_3);
            tracer.write_register(s_4_4 as isize, s_4_3);
        };
        // N s_4_6: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1u : u8
        let s_5_0: bool = true;
        // D s_5_1: write-var gs#34022 <= s_5_0
        fn_state.gs_34022 = s_5_0;
        // N s_5_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
