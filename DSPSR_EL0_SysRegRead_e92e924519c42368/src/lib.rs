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
use X_set::*;
use Halted::*;
use u__get_DSPSR_EL0::*;
use common::*;
pub fn DSPSR_EL0_SysRegRead_e92e924519c42368<T: Tracer>(
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
        ga_64548: ProductType5c790c8ef59cc8b2,
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
        // C s_1_1: const #102584u : u32
        let s_1_1: u32 = 102584;
        // D s_1_2: read-reg s_1_1:struct
        let s_1_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_1_1 as isize);
            tracer.read_register(s_1_1 as isize, value);
            value
        };
        // D s_1_3: call __get_DSPSR_EL0(s_1_2)
        let s_1_3: ProductType5c790c8ef59cc8b2 = u__get_DSPSR_EL0(state, tracer, s_1_2);
        // D s_1_4: write-var ga#64548 <= s_1_3
        fn_state.ga_64548 = s_1_3;
        // D s_1_5: read-var ga#64548.0:struct
        let s_1_5: u64 = fn_state.ga_64548._0;
        // D s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 64u16);
        // D s_1_7: read-var t:i
        let s_1_7: i128 = fn_state.t;
        // D s_1_8: call X_set(s_1_7, s_1_0, s_1_6)
        let s_1_8: () = X_set(state, tracer, s_1_7, s_1_0, s_1_6);
        // N s_1_9: return
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
