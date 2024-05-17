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
use Halted::*;
use X_read::*;
use Mk_DSPSR_EL0_Type::*;
use common::*;
pub fn DSPSR_EL0_SysRegWrite_2829d14e4c67df55<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    op0: u8,
    op1: u8,
    CRn: u8,
    op2: u8,
    CRm: u8,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        el: u8,
        op0: u8,
        op1: u8,
        CRn: u8,
        op2: u8,
        CRm: u8,
        t: i128,
    }
    let fn_state = FunctionState {
        el,
        op0,
        op1,
        CRn,
        op2,
        CRm,
        t,
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
        // S s_0_1: call Halted(s_0_0)
        let s_0_1: bool = Halted(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b2 b1
        if s_0_2 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #64s : i64
        let s_1_0: i64 = 64;
        // D s_1_1: read-var t:i
        let s_1_1: i128 = fn_state.t;
        // D s_1_2: call X_read(s_1_1, s_1_0)
        let s_1_2: Bits = X_read(state, tracer, s_1_1, s_1_0);
        // D s_1_3: cast reint s_1_2 -> u64
        let s_1_3: u64 = (s_1_2.value() as u64);
        // D s_1_4: call Mk_DSPSR_EL0_Type(s_1_3)
        let s_1_4: ProductType5c790c8ef59cc8b2 = Mk_DSPSR_EL0_Type(state, tracer, s_1_3);
        // C s_1_5: const #102584u : u32
        let s_1_5: u32 = 102584;
        // N s_1_6: write-reg s_1_5 <= s_1_4
        let s_1_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_1_5 as isize, s_1_4);
            tracer.write_register(s_1_5 as isize, s_1_4);
        };
        // N s_1_7: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: panic
        panic!("{:?}", ());
        // N s_2_1: return
        return;
    }
}
