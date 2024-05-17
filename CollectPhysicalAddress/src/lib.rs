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
use ProfilingBufferOwner::*;
use u_get_PMSCR_EL2_Type_PA::*;
use IsSecureEL2Enabled::*;
use StatisticalProfilingEnabled::*;
use u_get_PMSCR_EL1_Type_PA::*;
use common::*;
pub fn CollectPhysicalAddress<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_20028: (),
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_20034: bool,
        ga_15466: ProductTypec8897aad3eb4a29e,
        owning_ss: u32,
        return_value: bool,
        gs_20037: bool,
        owning_el: u8,
        gs_20035: bool,
        gs_20036: bool,
        gs_20028: (),
    }
    let fn_state = FunctionState {
        gs_20028,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call StatisticalProfilingEnabled(s_0_0)
        let s_0_1: bool = StatisticalProfilingEnabled(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b17 b1
        if s_0_2 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call ProfilingBufferOwner(s_1_0)
        let s_1_1: ProductTypec8897aad3eb4a29e = ProfilingBufferOwner(
            state,
            tracer,
            s_1_0,
        );
        // D s_1_2: write-var ga#15466 <= s_1_1
        fn_state.ga_15466 = s_1_1;
        // D s_1_3: read-var ga#15466.0:struct
        let s_1_3: u32 = fn_state.ga_15466._0;
        // D s_1_4: read-var ga#15466.1:struct
        let s_1_4: u8 = fn_state.ga_15466._1;
        // D s_1_5: write-var owning_ss <= s_1_3
        fn_state.owning_ss = s_1_3;
        // D s_1_6: write-var owning_el <= s_1_4
        fn_state.owning_el = s_1_4;
        // C s_1_7: const #432u : u32
        let s_1_7: u32 = 432;
        // D s_1_8: read-reg s_1_7:u8
        let s_1_8: u8 = {
            let value = state.read_register::<u8>(s_1_7 as isize);
            tracer.read_register(s_1_7 as isize, value);
            value
        };
        // C s_1_9: const #2u : u8
        let s_1_9: u8 = 2;
        // D s_1_10: cmp-lt s_1_8 s_1_9
        let s_1_10: bool = ((s_1_8) < (s_1_9));
        // N s_1_11: branch s_1_10 b13 b2
        if s_1_10 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#20035 <= s_2_0
        fn_state.gs_20035 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var gs#20035:u8
        let s_3_0: bool = fn_state.gs_20035;
        // N s_3_1: branch s_3_0 b6 b4
        if s_3_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #21072u : u32
        let s_4_0: u32 = 21072;
        // D s_4_1: read-reg s_4_0:struct
        let s_4_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: call _get_PMSCR_EL1_Type_PA(s_4_1)
        let s_4_2: bool = u_get_PMSCR_EL1_Type_PA(state, tracer, s_4_1);
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // C s_4_4: const #1u : u8
        let s_4_4: bool = true;
        // C s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 1u16);
        // D s_4_6: cmp-eq s_4_3 s_4_5
        let s_4_6: bool = ((s_4_3) == (s_4_5));
        // D s_4_7: write-var return_value <= s_4_6
        fn_state.return_value = s_4_6;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var return_value:u8
        let s_5_0: bool = fn_state.return_value;
        // N s_5_1: return s_5_0
        return s_5_0;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #104928u : u32
        let s_6_0: u32 = 104928;
        // D s_6_1: read-reg s_6_0:struct
        let s_6_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: call _get_PMSCR_EL2_Type_PA(s_6_1)
        let s_6_2: bool = u_get_PMSCR_EL2_Type_PA(state, tracer, s_6_1);
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // C s_6_4: const #1u : u8
        let s_6_4: bool = true;
        // C s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 1u16);
        // D s_6_6: cmp-eq s_6_3 s_6_5
        let s_6_6: bool = ((s_6_3) == (s_6_5));
        // N s_6_7: branch s_6_6 b9 b7
        if s_6_6 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#20037 <= s_7_0
        fn_state.gs_20037 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var gs#20037:u8
        let s_8_0: bool = fn_state.gs_20037;
        // D s_8_1: write-var return_value <= s_8_0
        fn_state.return_value = s_8_0;
        // N s_8_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var owning_el:u8
        let s_9_0: u8 = fn_state.owning_el;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 2u16);
        // C s_9_2: const #432u : u32
        let s_9_2: u32 = 432;
        // D s_9_3: read-reg s_9_2:u8
        let s_9_3: u8 = {
            let value = state.read_register::<u8>(s_9_2 as isize);
            tracer.read_register(s_9_2 as isize, value);
            value
        };
        // D s_9_4: cast zx s_9_3 -> bv
        let s_9_4: Bits = Bits::new(s_9_3 as u128, 2u16);
        // D s_9_5: cmp-eq s_9_1 s_9_4
        let s_9_5: bool = ((s_9_1) == (s_9_4));
        // N s_9_6: branch s_9_5 b12 b10
        if s_9_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #21072u : u32
        let s_10_0: u32 = 21072;
        // D s_10_1: read-reg s_10_0:struct
        let s_10_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: call _get_PMSCR_EL1_Type_PA(s_10_1)
        let s_10_2: bool = u_get_PMSCR_EL1_Type_PA(state, tracer, s_10_1);
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // C s_10_4: const #1u : u8
        let s_10_4: bool = true;
        // C s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 1u16);
        // D s_10_6: cmp-eq s_10_3 s_10_5
        let s_10_6: bool = ((s_10_3) == (s_10_5));
        // D s_10_7: write-var gs#20036 <= s_10_6
        fn_state.gs_20036 = s_10_6;
        // N s_10_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var gs#20036:u8
        let s_11_0: bool = fn_state.gs_20036;
        // D s_11_1: write-var gs#20037 <= s_11_0
        fn_state.gs_20037 = s_11_0;
        // N s_11_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#20036 <= s_12_0
        fn_state.gs_20036 = s_12_0;
        // N s_12_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var owning_ss:u32
        let s_13_0: u32 = fn_state.owning_ss;
        // C s_13_1: const #3u : u32
        let s_13_1: u32 = 3;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // N s_13_3: branch s_13_2 b16 b14
        if s_13_2 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call IsSecureEL2Enabled(s_14_0)
        let s_14_1: bool = IsSecureEL2Enabled(state, tracer, s_14_0);
        // D s_14_2: write-var gs#20034 <= s_14_1
        fn_state.gs_20034 = s_14_1;
        // N s_14_3: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_15_0: read-var gs#20034:u8
        let s_15_0: bool = fn_state.gs_20034;
        // D s_15_1: write-var gs#20035 <= s_15_0
        fn_state.gs_20035 = s_15_0;
        // N s_15_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#20034 <= s_16_0
        fn_state.gs_20034 = s_16_0;
        // N s_16_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var return_value <= s_17_0
        fn_state.return_value = s_17_0;
        // N s_17_2: jump b5
        return block_5(state, tracer, fn_state);
    }
}
