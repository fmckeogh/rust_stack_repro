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
use Mk_FPCRType::*;
use FPSCR_read__1::*;
use Zeros::*;
use u__get_FPSCR::*;
use common::*;
pub fn FPSCR_read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_1880: (),
) -> ProductType5c790c8ef59cc8b2 {
    #[derive(Default)]
    struct FunctionState {
        ga_917: ProductType700c18a878c5601b,
        gs_1880: (),
    }
    let fn_state = FunctionState {
        gs_1880,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_0_0: const #32s : i
        let s_0_0: i128 = 32;
        // S s_0_1: call Zeros(s_0_0)
        let s_0_1: Bits = Zeros(state, tracer, s_0_0);
        // S s_0_2: cast reint s_0_1 -> u32
        let s_0_2: u32 = (s_0_1.value() as u32);
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call FPSCR_read__1(s_0_3)
        let s_0_4: ProductType700c18a878c5601b = FPSCR_read__1(state, tracer, s_0_3);
        // S s_0_5: call __get_FPSCR(s_0_4)
        let s_0_5: ProductType700c18a878c5601b = u__get_FPSCR(state, tracer, s_0_4);
        // D s_0_6: write-var ga#917 <= s_0_5
        fn_state.ga_917 = s_0_5;
        // D s_0_7: read-var ga#917.0:struct
        let s_0_7: u32 = fn_state.ga_917._0;
        // S s_0_8: cast zx s_0_2 -> bv
        let s_0_8: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_9: cast zx s_0_7 -> bv
        let s_0_9: Bits = Bits::new(s_0_7 as u128, 32u16);
        // S s_0_10: cast reint s_0_8 -> u128
        let s_0_10: u128 = (s_0_8.value() as u128);
        // D s_0_11: size-of s_0_8
        let s_0_11: u16 = s_0_8.length();
        // D s_0_12: cast reint s_0_9 -> u128
        let s_0_12: u128 = (s_0_9.value() as u128);
        // D s_0_13: size-of s_0_9
        let s_0_13: u16 = s_0_9.length();
        // D s_0_14: lsl s_0_10 s_0_13
        let s_0_14: u128 = s_0_10 << s_0_13;
        // D s_0_15: or s_0_14 s_0_12
        let s_0_15: u128 = ((s_0_14) | (s_0_12));
        // D s_0_16: add s_0_11 s_0_13
        let s_0_16: u16 = (s_0_11 + s_0_13);
        // D s_0_17: create-bits s_0_15 s_0_16
        let s_0_17: Bits = Bits::new(s_0_15, s_0_16);
        // D s_0_18: cast reint s_0_17 -> u64
        let s_0_18: u64 = (s_0_17.value() as u64);
        // D s_0_19: tail-call Mk_FPCRType(s_0_18)
        return Mk_FPCRType(state, tracer, s_0_18);
    }
}
