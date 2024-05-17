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
use Bit::*;
use SPESampleAddOpOther__1::*;
use common::*;
pub fn SPESampleAddOpOther<T: Tracer>(
    state: &mut State,
    tracer: &T,
    conditional: bool,
    taken: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_19998: bool,
        gs_25539: bool,
        conditional: bool,
        taken: bool,
    }
    let fn_state = FunctionState {
        conditional,
        taken,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var conditional:u8
        let s_0_0: bool = fn_state.conditional;
        // N s_0_1: branch s_0_0 b6 b1
        if s_0_0 {
            return block_6(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#25539 <= s_1_0
        fn_state.gs_25539 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#25539:u8
        let s_2_0: bool = fn_state.gs_25539;
        // N s_2_1: branch s_2_0 b5 b3
        if s_2_0 {
            return block_5(state, tracer, fn_state);
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
        // D s_3_1: write-var ga#19998 <= s_3_0
        fn_state.ga_19998 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#19998:u8
        let s_4_0: bool = fn_state.ga_19998;
        // D s_4_1: call Bit(s_4_0)
        let s_4_1: bool = Bit(state, tracer, s_4_0);
        // C s_4_2: const #6s : i
        let s_4_2: i128 = 6;
        // C s_4_3: const #104856u : u32
        let s_4_3: u32 = 104856;
        // D s_4_4: read-reg s_4_3:u64
        let s_4_4: u64 = {
            let value = state.read_register::<u64>(s_4_3 as isize);
            tracer.read_register(s_4_3 as isize, value);
            value
        };
        // D s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 64u16);
        // C s_4_6: const #1u : u64
        let s_4_6: u64 = 1;
        // D s_4_7: bit-insert s_4_5 s_4_5 s_4_2 s_4_6
        let s_4_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_4_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_4_5.length(),
            );
            (s_4_5 & mask) | (s_4_5 << s_4_2)
        };
        // D s_4_8: cast reint s_4_7 -> u64
        let s_4_8: u64 = (s_4_7.value() as u64);
        // C s_4_9: const #104856u : u32
        let s_4_9: u32 = 104856;
        // N s_4_10: write-reg s_4_9 <= s_4_8
        let s_4_10: () = {
            state.write_register::<u64>(s_4_9 as isize, s_4_8);
            tracer.write_register(s_4_9 as isize, s_4_8);
        };
        // D s_4_11: read-var conditional:u8
        let s_4_11: bool = fn_state.conditional;
        // D s_4_12: call SPESampleAddOpOther__1(s_4_11)
        let s_4_12: () = SPESampleAddOpOther__1(state, tracer, s_4_11);
        // N s_4_13: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1u : u8
        let s_5_0: bool = true;
        // D s_5_1: write-var ga#19998 <= s_5_0
        fn_state.ga_19998 = s_5_0;
        // N s_5_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var taken:u8
        let s_6_0: bool = fn_state.taken;
        // D s_6_1: not s_6_0
        let s_6_1: bool = !s_6_0;
        // D s_6_2: write-var gs#25539 <= s_6_1
        fn_state.gs_25539 = s_6_1;
        // N s_6_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
