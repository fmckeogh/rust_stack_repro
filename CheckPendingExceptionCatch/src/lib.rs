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
use u_get_EDESR_Type_EC::*;
use Havev8p8Debug::*;
use Halt__1::*;
use HaltingAllowed::*;
use common::*;
pub fn CheckPendingExceptionCatch<T: Tracer>(
    state: &mut State,
    tracer: &T,
    is_async: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_29577: bool,
        gs_29578: bool,
        is_async: bool,
    }
    let fn_state = FunctionState {
        is_async,
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
        // S s_0_1: call Havev8p8Debug(s_0_0)
        let s_0_1: bool = Havev8p8Debug(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b8 b1
        if s_0_1 {
            return block_8(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#29577 <= s_1_0
        fn_state.gs_29577 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#29577:u8
        let s_2_0: bool = fn_state.gs_29577;
        // N s_2_1: branch s_2_0 b7 b3
        if s_2_0 {
            return block_7(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#29578 <= s_3_0
        fn_state.gs_29578 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#29578:u8
        let s_4_0: bool = fn_state.gs_29578;
        // N s_4_1: branch s_4_0 b6 b5
        if s_4_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1168u : u32
        let s_6_0: u32 = 1168;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: read-var is_async:u8
        let s_6_2: bool = fn_state.is_async;
        // D s_6_3: call Halt__1(s_6_1, s_6_2)
        let s_6_3: () = Halt__1(state, tracer, s_6_1, s_6_2);
        // N s_6_4: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #14712u : u32
        let s_7_0: u32 = 14712;
        // D s_7_1: read-reg s_7_0:struct
        let s_7_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call _get_EDESR_Type_EC(s_7_1)
        let s_7_2: bool = u_get_EDESR_Type_EC(state, tracer, s_7_1);
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // C s_7_4: const #1u : u8
        let s_7_4: bool = true;
        // C s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 1u16);
        // D s_7_6: cmp-eq s_7_3 s_7_5
        let s_7_6: bool = ((s_7_3) == (s_7_5));
        // D s_7_7: write-var gs#29578 <= s_7_6
        fn_state.gs_29578 = s_7_6;
        // N s_7_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call HaltingAllowed(s_8_0)
        let s_8_1: bool = HaltingAllowed(state, tracer, s_8_0);
        // D s_8_2: write-var gs#29577 <= s_8_1
        fn_state.gs_29577 = s_8_1;
        // N s_8_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
