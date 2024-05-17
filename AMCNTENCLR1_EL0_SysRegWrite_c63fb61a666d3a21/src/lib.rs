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
use Mk_AMCNTENCLR1_EL0_Type::*;
use IsHighestEL::*;
use X_read::*;
use common::*;
pub fn AMCNTENCLR1_EL0_SysRegWrite_c63fb61a666d3a21<T: Tracer>(
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
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call IsHighestEL(s_0_1)
        let s_0_2: bool = IsHighestEL(state, tracer, s_0_1);
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
        // N s_1_0: panic
        panic!("{:?}", ());
        // N s_1_1: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #64s : i64
        let s_2_0: i64 = 64;
        // D s_2_1: read-var t:i
        let s_2_1: i128 = fn_state.t;
        // D s_2_2: call X_read(s_2_1, s_2_0)
        let s_2_2: Bits = X_read(state, tracer, s_2_1, s_2_0);
        // D s_2_3: cast reint s_2_2 -> u64
        let s_2_3: u64 = (s_2_2.value() as u64);
        // D s_2_4: call Mk_AMCNTENCLR1_EL0_Type(s_2_3)
        let s_2_4: ProductType5c790c8ef59cc8b2 = Mk_AMCNTENCLR1_EL0_Type(
            state,
            tracer,
            s_2_3,
        );
        // C s_2_5: const #101920u : u32
        let s_2_5: u32 = 101920;
        // N s_2_6: write-reg s_2_5 <= s_2_4
        let s_2_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_2_5 as isize, s_2_4);
            tracer.write_register(s_2_5 as isize, s_2_4);
        };
        // N s_2_7: return
        return;
    }
}
