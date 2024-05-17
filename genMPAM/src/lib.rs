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
use u_get_MPAMHCR_EL2_Type_GSTAPP_PLK::*;
use genPMG::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use genPARTID::*;
use common::*;
pub fn genMPAM<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    InD: bool,
    InSM: bool,
    pspace: u32,
) -> ProductTypee79b4310dbe79c8c {
    #[derive(Default)]
    struct FunctionState {
        gs_7116: bool,
        returninfo: ProductTypee79b4310dbe79c8c,
        gs_7114: bool,
        ga_4843: ProductType4d3ef3a5cd661176,
        eff_el: u8,
        gs_7115: bool,
        el: u8,
        InD: bool,
        InSM: bool,
        pspace: u32,
    }
    let fn_state = FunctionState {
        el,
        InD,
        InSM,
        pspace,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_0_0: read-var el:u8
        let s_0_0: u8 = fn_state.el;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 2u16);
        // C s_0_2: const #448u : u32
        let s_0_2: u32 = 448;
        // D s_0_3: read-reg s_0_2:u8
        let s_0_3: u8 = {
            let value = state.read_register::<u8>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // D s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 2u16);
        // D s_0_5: cmp-eq s_0_1 s_0_4
        let s_0_5: bool = ((s_0_1) == (s_0_4));
        // N s_0_6: branch s_0_5 b12 b1
        if s_0_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#7114 <= s_1_0
        fn_state.gs_7114 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_2_0: read-var gs#7114:u8
        let s_2_0: bool = fn_state.gs_7114;
        // N s_2_1: branch s_2_0 b11 b3
        if s_2_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#7115 <= s_3_0
        fn_state.gs_7115 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_4_0: read-var gs#7115:u8
        let s_4_0: bool = fn_state.gs_7115;
        // N s_4_1: branch s_4_0 b10 b5
        if s_4_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#7116 <= s_5_0
        fn_state.gs_7116 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_6_0: read-var gs#7116:u8
        let s_6_0: bool = fn_state.gs_7116;
        // N s_6_1: branch s_6_0 b9 b7
        if s_6_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_7_0: read-var el:u8
        let s_7_0: u8 = fn_state.el;
        // D s_7_1: write-var eff_el <= s_7_0
        fn_state.eff_el = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_8_0: read-var eff_el:u8
        let s_8_0: u8 = fn_state.eff_el;
        // D s_8_1: read-var InD:u8
        let s_8_1: bool = fn_state.InD;
        // D s_8_2: read-var InSM:u8
        let s_8_2: bool = fn_state.InSM;
        // D s_8_3: call genPARTID(s_8_0, s_8_1, s_8_2)
        let s_8_3: ProductType4d3ef3a5cd661176 = genPARTID(
            state,
            tracer,
            s_8_0,
            s_8_1,
            s_8_2,
        );
        // D s_8_4: write-var ga#4843 <= s_8_3
        fn_state.ga_4843 = s_8_3;
        // D s_8_5: read-var ga#4843.0:struct
        let s_8_5: u16 = fn_state.ga_4843._0;
        // D s_8_6: read-var ga#4843.1:struct
        let s_8_6: bool = fn_state.ga_4843._1;
        // D s_8_7: read-var eff_el:u8
        let s_8_7: u8 = fn_state.eff_el;
        // D s_8_8: read-var InD:u8
        let s_8_8: bool = fn_state.InD;
        // D s_8_9: read-var InSM:u8
        let s_8_9: bool = fn_state.InSM;
        // D s_8_10: call genPMG(s_8_7, s_8_8, s_8_9, s_8_6)
        let s_8_10: u8 = genPMG(state, tracer, s_8_7, s_8_8, s_8_9, s_8_6);
        // D s_8_11: read-var pspace:u32
        let s_8_11: u32 = fn_state.pspace;
        // D s_8_12: write-var returninfo.0 <= s_8_11
        fn_state.returninfo._0 = s_8_11;
        // D s_8_13: write-var returninfo.1 <= s_8_5
        fn_state.returninfo._1 = s_8_5;
        // D s_8_14: write-var returninfo.2 <= s_8_10
        fn_state.returninfo._2 = s_8_10;
        // D s_8_15: read-var returninfo:struct
        let s_8_15: ProductTypee79b4310dbe79c8c = fn_state.returninfo;
        // N s_8_16: return s_8_15
        return s_8_15;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_9_0: const #440u : u32
        let s_9_0: u32 = 440;
        // D s_9_1: read-reg s_9_0:u8
        let s_9_1: u8 = {
            let value = state.read_register::<u8>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: write-var eff_el <= s_9_1
        fn_state.eff_el = s_9_1;
        // N s_9_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_10_0: const #102552u : u32
        let s_10_0: u32 = 102552;
        // D s_10_1: read-reg s_10_0:struct
        let s_10_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: call _get_HCR_EL2_Type_TGE(s_10_1)
        let s_10_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_10_1);
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // C s_10_4: const #0u : u8
        let s_10_4: bool = false;
        // C s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 1u16);
        // D s_10_6: cmp-eq s_10_3 s_10_5
        let s_10_6: bool = ((s_10_3) == (s_10_5));
        // D s_10_7: write-var gs#7116 <= s_10_6
        fn_state.gs_7116 = s_10_6;
        // N s_10_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_11_0: const #17608u : u32
        let s_11_0: u32 = 17608;
        // D s_11_1: read-reg s_11_0:struct
        let s_11_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: call _get_MPAMHCR_EL2_Type_GSTAPP_PLK(s_11_1)
        let s_11_2: bool = u_get_MPAMHCR_EL2_Type_GSTAPP_PLK(state, tracer, s_11_1);
        // D s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // C s_11_4: const #1u : u8
        let s_11_4: bool = true;
        // C s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 1u16);
        // D s_11_6: cmp-eq s_11_3 s_11_5
        let s_11_6: bool = ((s_11_3) == (s_11_5));
        // D s_11_7: write-var gs#7115 <= s_11_6
        fn_state.gs_7115 = s_11_6;
        // N s_11_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call EL2Enabled(s_12_0)
        let s_12_1: bool = EL2Enabled(state, tracer, s_12_0);
        // D s_12_2: write-var gs#7114 <= s_12_1
        fn_state.gs_7114 = s_12_1;
        // N s_12_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
