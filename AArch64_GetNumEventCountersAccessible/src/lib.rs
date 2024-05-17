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
use HaveFeatHPMN0::*;
use GetNumEventCounters::*;
use u_get_MDCR_EL2_Type_HPMN::*;
use EL2Enabled::*;
use ConstrainUnpredictableInteger::*;
use common::*;
pub fn AArch64_GetNumEventCountersAccessible<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_2022: (),
) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        gs_2029: bool,
        ga_1026: ProductType396b95aa74979079,
        n: i128,
        gs_2025: bool,
        total_counters: i128,
        gs_2024: bool,
        gs_2028: bool,
        gs_2022: (),
    }
    let fn_state = FunctionState {
        gs_2022,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call GetNumEventCounters(s_0_0)
        let s_0_1: i128 = GetNumEventCounters(state, tracer, s_0_0);
        // D s_0_2: write-var total_counters <= s_0_1
        fn_state.total_counters = s_0_1;
        // C s_0_3: const #16975u : u32
        let s_0_3: u32 = 16975;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // C s_0_6: const #440u : u32
        let s_0_6: u32 = 440;
        // D s_0_7: read-reg s_0_6:u8
        let s_0_7: u8 = {
            let value = state.read_register::<u8>(s_0_6 as isize);
            tracer.read_register(s_0_6 as isize, value);
            value
        };
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 2u16);
        // D s_0_9: cmp-eq s_0_5 s_0_8
        let s_0_9: bool = ((s_0_5) == (s_0_8));
        // N s_0_10: branch s_0_9 b18 b1
        if s_0_9 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #448u : u32
        let s_1_3: u32 = 448;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-eq s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) == (s_1_5));
        // D s_1_7: write-var gs#2024 <= s_1_6
        fn_state.gs_2024 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_2_0: read-var gs#2024:u8
        let s_2_0: bool = fn_state.gs_2024;
        // N s_2_1: branch s_2_0 b17 b3
        if s_2_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#2025 <= s_3_0
        fn_state.gs_2025 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_4_0: read-var gs#2025:u8
        let s_4_0: bool = fn_state.gs_2025;
        // N s_4_1: branch s_4_0 b7 b5
        if s_4_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_5_0: read-var total_counters:i
        let s_5_0: i128 = fn_state.total_counters;
        // D s_5_1: write-var n <= s_5_0
        fn_state.n = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_6_0: read-var n:i
        let s_6_0: i128 = fn_state.n;
        // N s_6_1: return s_6_0
        return s_6_0;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_7_0: const #104880u : u32
        let s_7_0: u32 = 104880;
        // D s_7_1: read-reg s_7_0:struct
        let s_7_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call _get_MDCR_EL2_Type_HPMN(s_7_1)
        let s_7_2: u8 = u_get_MDCR_EL2_Type_HPMN(state, tracer, s_7_1);
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 5u16);
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (s_7_3.value() as i128);
        // D s_7_5: write-var n <= s_7_4
        fn_state.n = s_7_4;
        // D s_7_6: read-var n:i
        let s_7_6: i128 = fn_state.n;
        // D s_7_7: read-var total_counters:i
        let s_7_7: i128 = fn_state.total_counters;
        // D s_7_8: cmp-gt s_7_6 s_7_7
        let s_7_8: bool = ((s_7_6) > (s_7_7));
        // N s_7_9: branch s_7_8 b16 b8
        if s_7_8 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call HaveFeatHPMN0(s_8_0)
        let s_8_1: bool = HaveFeatHPMN0(state, tracer, s_8_0);
        // S s_8_2: not s_8_1
        let s_8_2: bool = !s_8_1;
        // N s_8_3: branch s_8_2 b15 b9
        if s_8_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#2028 <= s_9_0
        fn_state.gs_2028 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_10_0: read-var gs#2028:u8
        let s_10_0: bool = fn_state.gs_2028;
        // D s_10_1: write-var gs#2029 <= s_10_0
        fn_state.gs_2029 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_11_0: read-var gs#2029:u8
        let s_11_0: bool = fn_state.gs_2029;
        // N s_11_1: branch s_11_0 b14 b12
        if s_11_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // N s_13_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_14_0: const #0s : i
        let s_14_0: i128 = 0;
        // D s_14_1: read-var total_counters:i
        let s_14_1: i128 = fn_state.total_counters;
        // C s_14_2: const #72u : u32
        let s_14_2: u32 = 72;
        // D s_14_3: call ConstrainUnpredictableInteger(s_14_0, s_14_1, s_14_2)
        let s_14_3: ProductType396b95aa74979079 = ConstrainUnpredictableInteger(
            state,
            tracer,
            s_14_0,
            s_14_1,
            s_14_2,
        );
        // D s_14_4: write-var ga#1026 <= s_14_3
        fn_state.ga_1026 = s_14_3;
        // D s_14_5: read-var ga#1026.1:struct
        let s_14_5: i128 = fn_state.ga_1026._1;
        // D s_14_6: write-var n <= s_14_5
        fn_state.n = s_14_5;
        // N s_14_7: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_15_0: const #0s : i
        let s_15_0: i128 = 0;
        // D s_15_1: read-var n:i
        let s_15_1: i128 = fn_state.n;
        // D s_15_2: cmp-eq s_15_1 s_15_0
        let s_15_2: bool = ((s_15_1) == (s_15_0));
        // D s_15_3: write-var gs#2028 <= s_15_2
        fn_state.gs_2028 = s_15_2;
        // N s_15_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#2029 <= s_16_0
        fn_state.gs_2029 = s_16_0;
        // N s_16_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call EL2Enabled(s_17_0)
        let s_17_1: bool = EL2Enabled(state, tracer, s_17_0);
        // D s_17_2: write-var gs#2025 <= s_17_1
        fn_state.gs_2025 = s_17_1;
        // N s_17_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#2024 <= s_18_0
        fn_state.gs_2024 = s_18_0;
        // N s_18_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
