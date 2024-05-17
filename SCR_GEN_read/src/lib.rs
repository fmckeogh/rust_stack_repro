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
use HaveAArch64::*;
use Mk_SCRType::*;
use common::*;
pub fn SCR_GEN_read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_1795: (),
) -> ProductType5c790c8ef59cc8b2 {
    #[derive(Default)]
    struct FunctionState {
        r: u64,
        gs_1795: (),
    }
    let fn_state = FunctionState {
        gs_1795,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_0_0: const #424u : u32
        let s_0_0: u32 = 424;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #2u : u8
        let s_0_2: u8 = 2;
        // D s_0_3: cmp-lt s_0_1 s_0_2
        let s_0_3: bool = ((s_0_1) < (s_0_2));
        // N s_0_4: assert s_0_3
        let s_0_4: () = assert!(s_0_3);
        // C s_0_5: const #() : ()
        let s_0_5: () = ();
        // S s_0_6: call HaveAArch64(s_0_5)
        let s_0_6: bool = HaveAArch64(state, tracer, s_0_5);
        // S s_0_7: not s_0_6
        let s_0_7: bool = !s_0_6;
        // N s_0_8: branch s_0_7 b3 b1
        if s_0_7 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_1_0: const #90704u : u32
        let s_1_0: u32 = 90704;
        // D s_1_1: read-reg s_1_0:u64
        let s_1_1: u64 = {
            let value = state.read_register::<u64>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: write-var r <= s_1_1
        fn_state.r = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_2_0: read-var r:u64
        let s_2_0: u64 = fn_state.r;
        // D s_2_1: tail-call Mk_SCRType(s_2_0)
        return Mk_SCRType(state, tracer, s_2_0);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_3_0: const #20920u : u32
        let s_3_0: u32 = 20920;
        // D s_3_1: read-reg s_3_0:u32
        let s_3_1: u32 = {
            let value = state.read_register::<u32>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // C s_3_2: const #64s : i
        let s_3_2: i128 = 64;
        // D s_3_3: cast zx s_3_1 -> bv
        let s_3_3: Bits = Bits::new(s_3_1 as u128, 32u16);
        // D s_3_4: bits-cast zx s_3_3 -> bv length s_3_2
        let s_3_4: Bits = s_3_3.zero_extend(s_3_2);
        // D s_3_5: cast reint s_3_4 -> u64
        let s_3_5: u64 = (s_3_4.value() as u64);
        // D s_3_6: write-var r <= s_3_5
        fn_state.r = s_3_5;
        // N s_3_7: jump b2
        return block_2(state, tracer, fn_state);
    }
}
