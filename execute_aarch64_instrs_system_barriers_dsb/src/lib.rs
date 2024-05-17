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
use SpeculativeStoreBypassBarrierToVA::*;
use FailTransaction::*;
use DataSynchronizationBarrier::*;
use IsHCRXEL2Enabled::*;
use HaveFeatXS::*;
use u_get_HCRX_EL2_Type_FnXS::*;
use SpeculativeStoreBypassBarrierToPA::*;
use HaveTME::*;
use Unreachable::*;
use common::*;
pub fn execute_aarch64_instrs_system_barriers_dsb<T: Tracer>(
    state: &mut State,
    tracer: &T,
    alias: u32,
    domain: u32,
    nXS__arg: bool,
    types: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_147876: bool,
        gs_147877: bool,
        nXS: bool,
        gs_147875: bool,
        gs_147872: bool,
        gs_147873: bool,
        alias: u32,
        domain: u32,
        nXS__arg: bool,
        types: u32,
    }
    let fn_state = FunctionState {
        alias,
        domain,
        nXS__arg,
        types,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var nXS__arg:u8
        let s_0_0: bool = fn_state.nXS__arg;
        // D s_0_1: write-var nXS <= s_0_0
        fn_state.nXS = s_0_0;
        // C s_0_2: const #0u : u32
        let s_0_2: u32 = 0;
        // D s_0_3: read-var alias:u32
        let s_0_3: u32 = fn_state.alias;
        // D s_0_4: cmp-eq s_0_2 s_0_3
        let s_0_4: bool = ((s_0_2) == (s_0_3));
        // D s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b2 b1
        if s_0_5 {
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
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call SpeculativeStoreBypassBarrierToVA(s_1_0)
        let s_1_1: () = SpeculativeStoreBypassBarrierToVA(state, tracer, s_1_0);
        // N s_1_2: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #1u : u32
        let s_2_0: u32 = 1;
        // D s_2_1: read-var alias:u32
        let s_2_1: u32 = fn_state.alias;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // D s_2_3: not s_2_2
        let s_2_3: bool = !s_2_2;
        // N s_2_4: branch s_2_3 b4 b3
        if s_2_3 {
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
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call SpeculativeStoreBypassBarrierToPA(s_3_0)
        let s_3_1: () = SpeculativeStoreBypassBarrierToPA(state, tracer, s_3_0);
        // N s_3_2: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #2u : u32
        let s_4_0: u32 = 2;
        // D s_4_1: read-var alias:u32
        let s_4_1: u32 = fn_state.alias;
        // D s_4_2: cmp-eq s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) == (s_4_1));
        // D s_4_3: not s_4_2
        let s_4_3: bool = !s_4_2;
        // N s_4_4: branch s_4_3 b27 b5
        if s_4_3 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call HaveTME(s_5_0)
        let s_5_1: bool = HaveTME(state, tracer, s_5_0);
        // N s_5_2: branch s_5_1 b26 b6
        if s_5_1 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#147872 <= s_6_0
        fn_state.gs_147872 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#147872:u8
        let s_7_0: bool = fn_state.gs_147872;
        // N s_7_1: branch s_7_0 b25 b8
        if s_7_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var nXS:u8
        let s_9_0: bool = fn_state.nXS;
        // D s_9_1: not s_9_0
        let s_9_1: bool = !s_9_0;
        // N s_9_2: branch s_9_1 b24 b10
        if s_9_1 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#147873 <= s_10_0
        fn_state.gs_147873 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#147873:u8
        let s_11_0: bool = fn_state.gs_147873;
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
    ) -> () {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var domain:u32
        let s_13_0: u32 = fn_state.domain;
        // D s_13_1: read-var types:u32
        let s_13_1: u32 = fn_state.types;
        // D s_13_2: read-var nXS:u8
        let s_13_2: bool = fn_state.nXS;
        // D s_13_3: call DataSynchronizationBarrier(s_13_0, s_13_1, s_13_2)
        let s_13_3: () = DataSynchronizationBarrier(
            state,
            tracer,
            s_13_0,
            s_13_1,
            s_13_2,
        );
        // N s_13_4: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #16975u : u32
        let s_14_0: u32 = 16975;
        // D s_14_1: read-reg s_14_0:u8
        let s_14_1: u8 = {
            let value = state.read_register::<u8>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 2u16);
        // C s_14_3: const #448u : u32
        let s_14_3: u32 = 448;
        // D s_14_4: read-reg s_14_3:u8
        let s_14_4: u8 = {
            let value = state.read_register::<u8>(s_14_3 as isize);
            tracer.read_register(s_14_3 as isize, value);
            value
        };
        // D s_14_5: cast zx s_14_4 -> bv
        let s_14_5: Bits = Bits::new(s_14_4 as u128, 2u16);
        // D s_14_6: cmp-eq s_14_2 s_14_5
        let s_14_6: bool = ((s_14_2) == (s_14_5));
        // N s_14_7: branch s_14_6 b23 b15
        if s_14_6 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #16975u : u32
        let s_15_0: u32 = 16975;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: u8 = {
            let value = state.read_register::<u8>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // D s_15_2: cast zx s_15_1 -> bv
        let s_15_2: Bits = Bits::new(s_15_1 as u128, 2u16);
        // C s_15_3: const #440u : u32
        let s_15_3: u32 = 440;
        // D s_15_4: read-reg s_15_3:u8
        let s_15_4: u8 = {
            let value = state.read_register::<u8>(s_15_3 as isize);
            tracer.read_register(s_15_3 as isize, value);
            value
        };
        // D s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 2u16);
        // D s_15_6: cmp-eq s_15_2 s_15_5
        let s_15_6: bool = ((s_15_2) == (s_15_5));
        // D s_15_7: write-var gs#147875 <= s_15_6
        fn_state.gs_147875 = s_15_6;
        // N s_15_8: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#147875:u8
        let s_16_0: bool = fn_state.gs_147875;
        // N s_16_1: branch s_16_0 b22 b17
        if s_16_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#147876 <= s_17_0
        fn_state.gs_147876 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#147876:u8
        let s_18_0: bool = fn_state.gs_147876;
        // N s_18_1: branch s_18_0 b21 b19
        if s_18_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#147877 <= s_19_0
        fn_state.gs_147877 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#147877:u8
        let s_20_0: bool = fn_state.gs_147877;
        // D s_20_1: write-var nXS <= s_20_0
        fn_state.nXS = s_20_0;
        // N s_20_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #22528u : u32
        let s_21_0: u32 = 22528;
        // D s_21_1: read-reg s_21_0:struct
        let s_21_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: call _get_HCRX_EL2_Type_FnXS(s_21_1)
        let s_21_2: bool = u_get_HCRX_EL2_Type_FnXS(state, tracer, s_21_1);
        // D s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // C s_21_4: const #1u : u8
        let s_21_4: bool = true;
        // C s_21_5: cast zx s_21_4 -> bv
        let s_21_5: Bits = Bits::new(s_21_4 as u128, 1u16);
        // D s_21_6: cmp-eq s_21_3 s_21_5
        let s_21_6: bool = ((s_21_3) == (s_21_5));
        // D s_21_7: write-var gs#147877 <= s_21_6
        fn_state.gs_147877 = s_21_6;
        // N s_21_8: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call IsHCRXEL2Enabled(s_22_0)
        let s_22_1: bool = IsHCRXEL2Enabled(state, tracer, s_22_0);
        // D s_22_2: write-var gs#147876 <= s_22_1
        fn_state.gs_147876 = s_22_1;
        // N s_22_3: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#147875 <= s_23_0
        fn_state.gs_147875 = s_23_0;
        // N s_23_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call HaveFeatXS(s_24_0)
        let s_24_1: bool = HaveFeatXS(state, tracer, s_24_0);
        // D s_24_2: write-var gs#147873 <= s_24_1
        fn_state.gs_147873 = s_24_1;
        // N s_24_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #2u : u32
        let s_25_0: u32 = 2;
        // C s_25_1: const #0u : u8
        let s_25_1: bool = false;
        // S s_25_2: call FailTransaction(s_25_0, s_25_1)
        let s_25_2: () = FailTransaction(state, tracer, s_25_0, s_25_1);
        // N s_25_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #100180u : u32
        let s_26_0: u32 = 100180;
        // D s_26_1: read-reg s_26_0:i
        let s_26_1: i128 = {
            let value = state.read_register::<i128>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // C s_26_2: const #0s : i
        let s_26_2: i128 = 0;
        // D s_26_3: cmp-gt s_26_1 s_26_2
        let s_26_3: bool = ((s_26_1) > (s_26_2));
        // D s_26_4: write-var gs#147872 <= s_26_3
        fn_state.gs_147872 = s_26_3;
        // N s_26_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call Unreachable(s_27_0)
        let s_27_1: () = Unreachable(state, tracer, s_27_0);
        // N s_27_2: return
        return;
    }
}
