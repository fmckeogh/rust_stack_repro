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
use GetNumEventCounters::*;
use u_get_MDCR_EL2_Type_HPMN::*;
use u_get_HDCR_Type_HPMN::*;
use ELUsingAArch32::*;
use HDCR_read::*;
use ConstrainUnpredictableInteger::*;
use HaveFeatHPMN0::*;
use EL2Enabled::*;
use common::*;
pub fn AArch32_GetNumEventCountersAccessible<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_2006: (),
) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        gs_2008: bool,
        ga_1009: u8,
        gs_2012: bool,
        n: i128,
        gs_2009: bool,
        total_counters: i128,
        ga_1014: ProductType396b95aa74979079,
        gs_2013: bool,
        gs_2006: (),
    }
    let fn_state = FunctionState {
        gs_2006,
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
        // N s_0_10: branch s_0_9 b21 b1
        if s_0_9 {
            return block_21(state, tracer, fn_state);
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
        // D s_1_7: write-var gs#2008 <= s_1_6
        fn_state.gs_2008 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_2_0: read-var gs#2008:u8
        let s_2_0: bool = fn_state.gs_2008;
        // N s_2_1: branch s_2_0 b20 b3
        if s_2_0 {
            return block_20(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#2009 <= s_3_0
        fn_state.gs_2009 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_4_0: read-var gs#2009:u8
        let s_4_0: bool = fn_state.gs_2009;
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
        // C s_7_0: const #432u : u32
        let s_7_0: u32 = 432;
        // D s_7_1: read-reg s_7_0:u8
        let s_7_1: u8 = {
            let value = state.read_register::<u8>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call ELUsingAArch32(s_7_1)
        let s_7_2: bool = ELUsingAArch32(state, tracer, s_7_1);
        // D s_7_3: not s_7_2
        let s_7_3: bool = !s_7_2;
        // N s_7_4: branch s_7_3 b19 b8
        if s_7_3 {
            return block_19(state, tracer, fn_state);
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
        // S s_8_1: call HDCR_read(s_8_0)
        let s_8_1: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_8_0);
        // S s_8_2: call _get_HDCR_Type_HPMN(s_8_1)
        let s_8_2: u8 = u_get_HDCR_Type_HPMN(state, tracer, s_8_1);
        // D s_8_3: write-var ga#1009 <= s_8_2
        fn_state.ga_1009 = s_8_2;
        // N s_8_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_9_0: read-var ga#1009:u8
        let s_9_0: u8 = fn_state.ga_1009;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 5u16);
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (s_9_1.value() as i128);
        // D s_9_3: write-var n <= s_9_2
        fn_state.n = s_9_2;
        // D s_9_4: read-var n:i
        let s_9_4: i128 = fn_state.n;
        // D s_9_5: read-var total_counters:i
        let s_9_5: i128 = fn_state.total_counters;
        // D s_9_6: cmp-gt s_9_4 s_9_5
        let s_9_6: bool = ((s_9_4) > (s_9_5));
        // N s_9_7: branch s_9_6 b18 b10
        if s_9_6 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call HaveFeatHPMN0(s_10_0)
        let s_10_1: bool = HaveFeatHPMN0(state, tracer, s_10_0);
        // S s_10_2: not s_10_1
        let s_10_2: bool = !s_10_1;
        // N s_10_3: branch s_10_2 b17 b11
        if s_10_2 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#2012 <= s_11_0
        fn_state.gs_2012 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_12_0: read-var gs#2012:u8
        let s_12_0: bool = fn_state.gs_2012;
        // D s_12_1: write-var gs#2013 <= s_12_0
        fn_state.gs_2013 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_13_0: read-var gs#2013:u8
        let s_13_0: bool = fn_state.gs_2013;
        // N s_13_1: branch s_13_0 b16 b14
        if s_13_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // N s_15_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_16_0: const #0s : i
        let s_16_0: i128 = 0;
        // D s_16_1: read-var total_counters:i
        let s_16_1: i128 = fn_state.total_counters;
        // C s_16_2: const #72u : u32
        let s_16_2: u32 = 72;
        // D s_16_3: call ConstrainUnpredictableInteger(s_16_0, s_16_1, s_16_2)
        let s_16_3: ProductType396b95aa74979079 = ConstrainUnpredictableInteger(
            state,
            tracer,
            s_16_0,
            s_16_1,
            s_16_2,
        );
        // D s_16_4: write-var ga#1014 <= s_16_3
        fn_state.ga_1014 = s_16_3;
        // D s_16_5: read-var ga#1014.1:struct
        let s_16_5: i128 = fn_state.ga_1014._1;
        // D s_16_6: write-var n <= s_16_5
        fn_state.n = s_16_5;
        // N s_16_7: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_17_0: const #0s : i
        let s_17_0: i128 = 0;
        // D s_17_1: read-var n:i
        let s_17_1: i128 = fn_state.n;
        // D s_17_2: cmp-eq s_17_1 s_17_0
        let s_17_2: bool = ((s_17_1) == (s_17_0));
        // D s_17_3: write-var gs#2012 <= s_17_2
        fn_state.gs_2012 = s_17_2;
        // N s_17_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#2013 <= s_18_0
        fn_state.gs_2013 = s_18_0;
        // N s_18_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_19_0: const #104880u : u32
        let s_19_0: u32 = 104880;
        // D s_19_1: read-reg s_19_0:struct
        let s_19_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call _get_MDCR_EL2_Type_HPMN(s_19_1)
        let s_19_2: u8 = u_get_MDCR_EL2_Type_HPMN(state, tracer, s_19_1);
        // D s_19_3: write-var ga#1009 <= s_19_2
        fn_state.ga_1009 = s_19_2;
        // N s_19_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call EL2Enabled(s_20_0)
        let s_20_1: bool = EL2Enabled(state, tracer, s_20_0);
        // D s_20_2: write-var gs#2009 <= s_20_1
        fn_state.gs_2009 = s_20_1;
        // N s_20_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#2008 <= s_21_0
        fn_state.gs_2008 = s_21_0;
        // N s_21_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
