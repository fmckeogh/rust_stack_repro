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
use common::*;
pub fn SPESampleAddOpOther__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    conditional: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_20000: bool,
        conditional: bool,
    }
    let fn_state = FunctionState {
        conditional,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0u : u8
        let s_0_0: u8 = 0;
        // C s_0_1: const #17136u : u32
        let s_0_1: u32 = 17136;
        // N s_0_2: write-reg s_0_1 <= s_0_0
        let s_0_2: () = {
            state.write_register::<u8>(s_0_1 as isize, s_0_0);
            tracer.write_register(s_0_1 as isize, s_0_0);
        };
        // D s_0_3: read-var conditional:u8
        let s_0_3: bool = fn_state.conditional;
        // N s_0_4: branch s_0_3 b3 b1
        if s_0_3 {
            return block_3(state, tracer, fn_state);
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
        // D s_1_1: write-var ga#20000 <= s_1_0
        fn_state.ga_20000 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#20000:u8
        let s_2_0: bool = fn_state.ga_20000;
        // D s_2_1: call Bit(s_2_0)
        let s_2_1: bool = Bit(state, tracer, s_2_0);
        // C s_2_2: const #0s : i
        let s_2_2: i128 = 0;
        // C s_2_3: const #13528u : u32
        let s_2_3: u32 = 13528;
        // D s_2_4: read-reg s_2_3:u8
        let s_2_4: u8 = {
            let value = state.read_register::<u8>(s_2_3 as isize);
            tracer.read_register(s_2_3 as isize, value);
            value
        };
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 8u16);
        // C s_2_6: const #1u : u64
        let s_2_6: u64 = 1;
        // D s_2_7: bit-insert s_2_5 s_2_5 s_2_2 s_2_6
        let s_2_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_2_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_2_5.length(),
            );
            (s_2_5 & mask) | (s_2_5 << s_2_2)
        };
        // D s_2_8: cast reint s_2_7 -> u8
        let s_2_8: u8 = (s_2_7.value() as u8);
        // C s_2_9: const #13528u : u32
        let s_2_9: u32 = 13528;
        // N s_2_10: write-reg s_2_9 <= s_2_8
        let s_2_10: () = {
            state.write_register::<u8>(s_2_9 as isize, s_2_8);
            tracer.write_register(s_2_9 as isize, s_2_8);
        };
        // C s_2_11: const #4u : u32
        let s_2_11: u32 = 4;
        // C s_2_12: const #19040u : u32
        let s_2_12: u32 = 19040;
        // N s_2_13: write-reg s_2_12 <= s_2_11
        let s_2_13: () = {
            state.write_register::<u32>(s_2_12 as isize, s_2_11);
            tracer.write_register(s_2_12 as isize, s_2_11);
        };
        // N s_2_14: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // D s_3_1: write-var ga#20000 <= s_3_0
        fn_state.ga_20000 = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
