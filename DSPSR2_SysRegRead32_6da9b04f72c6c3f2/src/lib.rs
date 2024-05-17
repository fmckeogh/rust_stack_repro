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
use DSPSR2_read::*;
use u__get_DSPSR2::*;
use R_set::*;
use Halted::*;
use common::*;
pub fn DSPSR2_SysRegRead32_6da9b04f72c6c3f2<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    coproc: u8,
    opc1: u8,
    CRn: u8,
    opc2: u8,
    CRm: u8,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_176763: ProductType700c18a878c5601b,
        el: u8,
        coproc: u8,
        opc1: u8,
        CRn: u8,
        opc2: u8,
        CRm: u8,
        t: i128,
    }
    let fn_state = FunctionState {
        el,
        coproc,
        opc1,
        CRn,
        opc2,
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
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call DSPSR2_read(s_1_0)
        let s_1_1: ProductType700c18a878c5601b = DSPSR2_read(state, tracer, s_1_0);
        // S s_1_2: call __get_DSPSR2(s_1_1)
        let s_1_2: ProductType700c18a878c5601b = u__get_DSPSR2(state, tracer, s_1_1);
        // D s_1_3: write-var ga#176763 <= s_1_2
        fn_state.ga_176763 = s_1_2;
        // D s_1_4: read-var ga#176763.0:struct
        let s_1_4: u32 = fn_state.ga_176763._0;
        // D s_1_5: read-var t:i
        let s_1_5: i128 = fn_state.t;
        // D s_1_6: call R_set(s_1_5, s_1_4)
        let s_1_6: () = R_set(state, tracer, s_1_5, s_1_4);
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
