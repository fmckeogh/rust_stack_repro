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
use AArch32_SystemAccessTrapSyndrome::*;
use ThisInstr::*;
use ELFromM32::*;
use AArch32_TakeUndefInstrException::*;
use AArch32_TakeHypTrapException__1::*;
use common::*;
pub fn AArch32_SystemAccessTrap<T: Tracer>(
    state: &mut State,
    tracer: &T,
    mode: u8,
    ec: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_30755: bool,
        ga_23925: ProductTypea5cc8de4daab131c,
        gs_30754: bool,
        gs_30756: bool,
        target_el: u8,
        mode: u8,
        ec: i128,
    }
    let fn_state = FunctionState {
        mode,
        ec,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var mode:u8
        let s_0_0: u8 = fn_state.mode;
        // D s_0_1: call ELFromM32(s_0_0)
        let s_0_1: ProductTypea5cc8de4daab131c = ELFromM32(state, tracer, s_0_0);
        // D s_0_2: write-var ga#23925 <= s_0_1
        fn_state.ga_23925 = s_0_1;
        // D s_0_3: read-var ga#23925.0:struct
        let s_0_3: bool = fn_state.ga_23925._0;
        // D s_0_4: read-var ga#23925.1:struct
        let s_0_4: u8 = fn_state.ga_23925._1;
        // D s_0_5: write-var target_el <= s_0_4
        fn_state.target_el = s_0_4;
        // N s_0_6: branch s_0_3 b11 b1
        if s_0_3 {
            return block_11(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#30754 <= s_1_0
        fn_state.gs_30754 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#30754:u8
        let s_2_0: bool = fn_state.gs_30754;
        // N s_2_1: branch s_2_0 b10 b3
        if s_2_0 {
            return block_10(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#30755 <= s_3_0
        fn_state.gs_30755 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#30755:u8
        let s_4_0: bool = fn_state.gs_30755;
        // N s_4_1: branch s_4_0 b9 b5
        if s_4_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#30756 <= s_5_0
        fn_state.gs_30756 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#30756:u8
        let s_6_0: bool = fn_state.gs_30756;
        // N s_6_1: assert s_6_0
        let s_6_1: () = assert!(s_6_0);
        // D s_6_2: read-var target_el:u8
        let s_6_2: u8 = fn_state.target_el;
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 2u16);
        // C s_6_4: const #432u : u32
        let s_6_4: u32 = 432;
        // D s_6_5: read-reg s_6_4:u8
        let s_6_5: u8 = {
            let value = state.read_register::<u8>(s_6_4 as isize);
            tracer.read_register(s_6_4 as isize, value);
            value
        };
        // D s_6_6: cast zx s_6_5 -> bv
        let s_6_6: Bits = Bits::new(s_6_5 as u128, 2u16);
        // D s_6_7: cmp-eq s_6_3 s_6_6
        let s_6_7: bool = ((s_6_3) == (s_6_6));
        // N s_6_8: branch s_6_7 b8 b7
        if s_6_7 {
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
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call AArch32_TakeUndefInstrException(s_7_0)
        let s_7_1: () = AArch32_TakeUndefInstrException(state, tracer, s_7_0);
        // N s_7_2: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call ThisInstr(s_8_0)
        let s_8_1: u32 = ThisInstr(state, tracer, s_8_0);
        // D s_8_2: read-var ec:i
        let s_8_2: i128 = fn_state.ec;
        // D s_8_3: call AArch32_SystemAccessTrapSyndrome(s_8_1, s_8_2)
        let s_8_3: ProductTypeb7f99f96751e17c4 = AArch32_SystemAccessTrapSyndrome(
            state,
            tracer,
            s_8_1,
            s_8_2,
        );
        // D s_8_4: call AArch32_TakeHypTrapException__1(s_8_3)
        let s_8_4: () = AArch32_TakeHypTrapException__1(state, tracer, s_8_3);
        // N s_8_5: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var target_el:u8
        let s_9_0: u8 = fn_state.target_el;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 2u16);
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (s_9_1.value() as i128);
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // C s_9_4: const #16975u : u32
        let s_9_4: u32 = 16975;
        // D s_9_5: read-reg s_9_4:u8
        let s_9_5: u8 = {
            let value = state.read_register::<u8>(s_9_4 as isize);
            tracer.read_register(s_9_4 as isize, value);
            value
        };
        // D s_9_6: cast zx s_9_5 -> bv
        let s_9_6: Bits = Bits::new(s_9_5 as u128, 2u16);
        // D s_9_7: cast zx s_9_6 -> i
        let s_9_7: i128 = (s_9_6.value() as i128);
        // D s_9_8: cast reint s_9_7 -> i64
        let s_9_8: i64 = (s_9_7 as i64);
        // D s_9_9: cast zx s_9_3 -> i
        let s_9_9: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_10: cast zx s_9_8 -> i
        let s_9_10: i128 = (i128::try_from(s_9_8).unwrap());
        // D s_9_11: cmp-ge s_9_9 s_9_10
        let s_9_11: bool = ((s_9_9) >= (s_9_10));
        // D s_9_12: write-var gs#30756 <= s_9_11
        fn_state.gs_30756 = s_9_11;
        // N s_9_13: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var target_el:u8
        let s_10_0: u8 = fn_state.target_el;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 2u16);
        // C s_10_2: const #448u : u32
        let s_10_2: u32 = 448;
        // D s_10_3: read-reg s_10_2:u8
        let s_10_3: u8 = {
            let value = state.read_register::<u8>(s_10_2 as isize);
            tracer.read_register(s_10_2 as isize, value);
            value
        };
        // D s_10_4: cast zx s_10_3 -> bv
        let s_10_4: Bits = Bits::new(s_10_3 as u128, 2u16);
        // D s_10_5: cmp-ne s_10_1 s_10_4
        let s_10_5: bool = ((s_10_1) != (s_10_4));
        // D s_10_6: write-var gs#30755 <= s_10_5
        fn_state.gs_30755 = s_10_5;
        // N s_10_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var target_el:u8
        let s_11_0: u8 = fn_state.target_el;
        // C s_11_1: const #2u : u8
        let s_11_1: u8 = 2;
        // D s_11_2: cmp-lt s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) < (s_11_1));
        // D s_11_3: write-var gs#30754 <= s_11_2
        fn_state.gs_30754 = s_11_2;
        // N s_11_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
