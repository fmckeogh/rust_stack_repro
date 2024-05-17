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
use HavePFAR::*;
use Mk_PFAR_EL2_Type::*;
use Mk_MFAR_EL3_Type::*;
use Unreachable::*;
use HaveRME::*;
use Mk_PFAR_EL1_Type::*;
use common::*;
pub fn PFAR_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regime: u8,
    value_name: u64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: u64,
        gs_5822: bool,
        gs_5821: bool,
        regime: u8,
        value_name: u64,
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
        // D s_0_0: read-var value_name:u64
        let s_0_0: u64 = fn_state.value_name;
        // D s_0_1: write-var r <= s_0_0
        fn_state.r = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call HavePFAR(s_0_2)
        let s_0_3: bool = HavePFAR(state, tracer, s_0_2);
        // N s_0_4: branch s_0_3 b12 b1
        if s_0_3 {
            return block_12(state, tracer, fn_state);
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
        // S s_1_1: call HaveRME(s_1_0)
        let s_1_1: bool = HaveRME(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b11 b2
        if s_1_1 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#5821 <= s_2_0
        fn_state.gs_5821 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#5821:u8
        let s_3_0: bool = fn_state.gs_5821;
        // D s_3_1: write-var gs#5822 <= s_3_0
        fn_state.gs_5822 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#5822:u8
        let s_4_0: bool = fn_state.gs_5822;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // D s_4_2: read-var regime:u8
        let s_4_2: u8 = fn_state.regime;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 2u16);
        // C s_4_4: const #440u : u32
        let s_4_4: u32 = 440;
        // D s_4_5: read-reg s_4_4:u8
        let s_4_5: u8 = {
            let value = state.read_register::<u8>(s_4_4 as isize);
            tracer.read_register(s_4_4 as isize, value);
            value
        };
        // D s_4_6: cast zx s_4_5 -> bv
        let s_4_6: Bits = Bits::new(s_4_5 as u128, 2u16);
        // D s_4_7: cmp-eq s_4_3 s_4_6
        let s_4_7: bool = ((s_4_3) == (s_4_6));
        // D s_4_8: not s_4_7
        let s_4_8: bool = !s_4_7;
        // N s_4_9: branch s_4_8 b6 b5
        if s_4_8 {
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
        // D s_5_1: call Mk_PFAR_EL1_Type(s_5_0)
        let s_5_1: ProductType5c790c8ef59cc8b2 = Mk_PFAR_EL1_Type(state, tracer, s_5_0);
        // C s_5_2: const #102840u : u32
        let s_5_2: u32 = 102840;
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
        // D s_6_0: read-var regime:u8
        let s_6_0: u8 = fn_state.regime;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #432u : u32
        let s_6_2: u32 = 432;
        // D s_6_3: read-reg s_6_2:u8
        let s_6_3: u8 = {
            let value = state.read_register::<u8>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 2u16);
        // D s_6_5: cmp-eq s_6_1 s_6_4
        let s_6_5: bool = ((s_6_1) == (s_6_4));
        // D s_6_6: not s_6_5
        let s_6_6: bool = !s_6_5;
        // N s_6_7: branch s_6_6 b8 b7
        if s_6_6 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var r:u64
        let s_7_0: u64 = fn_state.r;
        // D s_7_1: call Mk_PFAR_EL2_Type(s_7_0)
        let s_7_1: ProductType5c790c8ef59cc8b2 = Mk_PFAR_EL2_Type(state, tracer, s_7_0);
        // C s_7_2: const #20008u : u32
        let s_7_2: u32 = 20008;
        // N s_7_3: write-reg s_7_2 <= s_7_1
        let s_7_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_7_2 as isize, s_7_1);
            tracer.write_register(s_7_2 as isize, s_7_1);
        };
        // N s_7_4: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var regime:u8
        let s_8_0: u8 = fn_state.regime;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 2u16);
        // C s_8_2: const #424u : u32
        let s_8_2: u32 = 424;
        // D s_8_3: read-reg s_8_2:u8
        let s_8_3: u8 = {
            let value = state.read_register::<u8>(s_8_2 as isize);
            tracer.read_register(s_8_2 as isize, value);
            value
        };
        // D s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 2u16);
        // D s_8_5: cmp-eq s_8_1 s_8_4
        let s_8_5: bool = ((s_8_1) == (s_8_4));
        // D s_8_6: not s_8_5
        let s_8_6: bool = !s_8_5;
        // N s_8_7: branch s_8_6 b10 b9
        if s_8_6 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var r:u64
        let s_9_0: u64 = fn_state.r;
        // D s_9_1: call Mk_MFAR_EL3_Type(s_9_0)
        let s_9_1: ProductType5c790c8ef59cc8b2 = Mk_MFAR_EL3_Type(state, tracer, s_9_0);
        // C s_9_2: const #100280u : u32
        let s_9_2: u32 = 100280;
        // N s_9_3: write-reg s_9_2 <= s_9_1
        let s_9_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_9_2 as isize, s_9_1);
            tracer.write_register(s_9_2 as isize, s_9_1);
        };
        // N s_9_4: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call Unreachable(s_10_0)
        let s_10_1: () = Unreachable(state, tracer, s_10_0);
        // N s_10_2: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var regime:u8
        let s_11_0: u8 = fn_state.regime;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 2u16);
        // C s_11_2: const #424u : u32
        let s_11_2: u32 = 424;
        // D s_11_3: read-reg s_11_2:u8
        let s_11_3: u8 = {
            let value = state.read_register::<u8>(s_11_2 as isize);
            tracer.read_register(s_11_2 as isize, value);
            value
        };
        // D s_11_4: cast zx s_11_3 -> bv
        let s_11_4: Bits = Bits::new(s_11_3 as u128, 2u16);
        // D s_11_5: cmp-eq s_11_1 s_11_4
        let s_11_5: bool = ((s_11_1) == (s_11_4));
        // D s_11_6: write-var gs#5821 <= s_11_5
        fn_state.gs_5821 = s_11_5;
        // N s_11_7: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#5822 <= s_12_0
        fn_state.gs_5822 = s_12_0;
        // N s_12_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
