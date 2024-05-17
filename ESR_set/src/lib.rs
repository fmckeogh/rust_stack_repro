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
use Mk_ESR_EL3_Type::*;
use Mk_ESR_EL2_Type::*;
use Mk_ESR_EL1_Type::*;
use Unreachable::*;
use common::*;
pub fn ESR_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regime: u8,
    value_name: ProductType5c790c8ef59cc8b2,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: u64,
        regime: u8,
        value_name: ProductType5c790c8ef59cc8b2,
    }
    let fn_state = FunctionState {
        regime,
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var value_name.0:struct
        let s_0_0: u64 = fn_state.value_name._0;
        // D s_0_1: write-var r <= s_0_0
        fn_state.r = s_0_0;
        // D s_0_2: read-var regime:u8
        let s_0_2: u8 = fn_state.regime;
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 2u16);
        // C s_0_4: const #440u : u32
        let s_0_4: u32 = 440;
        // D s_0_5: read-reg s_0_4:u8
        let s_0_5: u8 = {
            let value = state.read_register::<u8>(s_0_4 as isize);
            tracer.read_register(s_0_4 as isize, value);
            value
        };
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 2u16);
        // D s_0_7: cmp-eq s_0_3 s_0_6
        let s_0_7: bool = ((s_0_3) == (s_0_6));
        // D s_0_8: not s_0_7
        let s_0_8: bool = !s_0_7;
        // N s_0_9: branch s_0_8 b2 b1
        if s_0_8 {
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
        // D s_1_0: read-var r:u64
        let s_1_0: u64 = fn_state.r;
        // D s_1_1: call Mk_ESR_EL1_Type(s_1_0)
        let s_1_1: ProductType5c790c8ef59cc8b2 = Mk_ESR_EL1_Type(state, tracer, s_1_0);
        // C s_1_2: const #22952u : u32
        let s_1_2: u32 = 22952;
        // N s_1_3: write-reg s_1_2 <= s_1_1
        let s_1_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_1_2 as isize, s_1_1);
            tracer.write_register(s_1_2 as isize, s_1_1);
        };
        // N s_1_4: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var regime:u8
        let s_2_0: u8 = fn_state.regime;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #432u : u32
        let s_2_2: u32 = 432;
        // D s_2_3: read-reg s_2_2:u8
        let s_2_3: u8 = {
            let value = state.read_register::<u8>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 2u16);
        // D s_2_5: cmp-eq s_2_1 s_2_4
        let s_2_5: bool = ((s_2_1) == (s_2_4));
        // D s_2_6: not s_2_5
        let s_2_6: bool = !s_2_5;
        // N s_2_7: branch s_2_6 b4 b3
        if s_2_6 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var r:u64
        let s_3_0: u64 = fn_state.r;
        // D s_3_1: call Mk_ESR_EL2_Type(s_3_0)
        let s_3_1: ProductType5c790c8ef59cc8b2 = Mk_ESR_EL2_Type(state, tracer, s_3_0);
        // C s_3_2: const #90688u : u32
        let s_3_2: u32 = 90688;
        // N s_3_3: write-reg s_3_2 <= s_3_1
        let s_3_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_3_2 as isize, s_3_1);
            tracer.write_register(s_3_2 as isize, s_3_1);
        };
        // N s_3_4: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var regime:u8
        let s_4_0: u8 = fn_state.regime;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #424u : u32
        let s_4_2: u32 = 424;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 2u16);
        // D s_4_5: cmp-eq s_4_1 s_4_4
        let s_4_5: bool = ((s_4_1) == (s_4_4));
        // D s_4_6: not s_4_5
        let s_4_6: bool = !s_4_5;
        // N s_4_7: branch s_4_6 b6 b5
        if s_4_6 {
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
        // D s_5_0: read-var r:u64
        let s_5_0: u64 = fn_state.r;
        // D s_5_1: call Mk_ESR_EL3_Type(s_5_0)
        let s_5_1: ProductType5c790c8ef59cc8b2 = Mk_ESR_EL3_Type(state, tracer, s_5_0);
        // C s_5_2: const #11992u : u32
        let s_5_2: u32 = 11992;
        // N s_5_3: write-reg s_5_2 <= s_5_1
        let s_5_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_5_2 as isize, s_5_1);
            tracer.write_register(s_5_2 as isize, s_5_1);
        };
        // N s_5_4: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call Unreachable(s_6_0)
        let s_6_1: () = Unreachable(state, tracer, s_6_0);
        // N s_6_2: return
        return;
    }
}
