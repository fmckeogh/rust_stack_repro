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
use GetTimestamp::*;
use Mk_BRBTS_EL1_Type::*;
use BRBETimeStamp::*;
use common::*;
pub fn BRBEFreeze<T: Tracer>(state: &mut State, tracer: &T, gs_2930: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_2930: (),
    }
    let fn_state = FunctionState {
        gs_2930,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16536u : u32
        let s_0_0: u32 = 16536;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #16536u : u32
        let s_0_2: u32 = 16536;
        // N s_0_3: write-reg s_0_2 <= s_0_1
        let s_0_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_2 as isize, s_0_1);
            tracer.write_register(s_0_2 as isize, s_0_1);
        };
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call BRBETimeStamp(s_0_4)
        let s_0_5: u32 = BRBETimeStamp(state, tracer, s_0_4);
        // S s_0_6: call GetTimestamp(s_0_5)
        let s_0_6: u64 = GetTimestamp(state, tracer, s_0_5);
        // S s_0_7: call Mk_BRBTS_EL1_Type(s_0_6)
        let s_0_7: ProductType5c790c8ef59cc8b2 = Mk_BRBTS_EL1_Type(state, tracer, s_0_6);
        // C s_0_8: const #23008u : u32
        let s_0_8: u32 = 23008;
        // N s_0_9: write-reg s_0_8 <= s_0_7
        let s_0_9: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_8 as isize, s_0_7);
            tracer.write_register(s_0_8 as isize, s_0_7);
        };
        // N s_0_10: return
        return;
    }
}
