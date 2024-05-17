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
use u_get_MPAMIDR_EL1_Type_HAS_SDEFLT::*;
use genMPAM::*;
use HaveRME::*;
use DefaultMPAMinfo::*;
use MPAM3_EL3_read::*;
use AltPARTIDspace::*;
use u__IMPDEF_boolean::*;
use u_get_MPAM3_EL3_Type_FORCE_NS::*;
use HaveSME::*;
use PARTIDspaceFromSS::*;
use MPAMisEnabled::*;
use HaveMPAMv0p1Ext::*;
use u_get_MPAM3_EL3_Type_SDEFLT::*;
use ELFromM32::*;
use SecurityStateAtEL::*;
use UsingAArch32::*;
use HaveMPAMv1p1Ext::*;
use u_get_MPAMIDR_EL1_Type_HAS_FORCE_NS::*;
use u_get_MPAMIDR_EL1_Type_HAS_ALTSP::*;
use common::*;
pub fn GenMPAMatEL<T: Tracer>(
    state: &mut State,
    tracer: &T,
    acctype: u32,
    el: u8,
) -> ProductTypee79b4310dbe79c8c {
    #[derive(Default)]
    struct FunctionState {
        gs_7124: bool,
        gs_7141: bool,
        gs_7128: bool,
        return_value: ProductTypee79b4310dbe79c8c,
        gs_7132: bool,
        ga_4855: ProductTypea5cc8de4daab131c,
        gs_7130: bool,
        gs_7143: bool,
        gs_7135: bool,
        gs_7131: bool,
        security: u32,
        mpamEL: u8,
        InD: bool,
        gs_7136: bool,
        InSM: bool,
        gs_7146: bool,
        pspace: u32,
        gs_7140: bool,
        validEL_name: bool,
        gs_7134: bool,
        gs_7139: bool,
        gs_7149: bool,
        acctype: u32,
        el: u8,
    }
    let fn_state = FunctionState {
        acctype,
        el,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_0_0: const #0u : u8
        let s_0_0: bool = false;
        // D s_0_1: write-var validEL_name <= s_0_0
        fn_state.validEL_name = s_0_0;
        // D s_0_2: read-var el:u8
        let s_0_2: u8 = fn_state.el;
        // D s_0_3: call SecurityStateAtEL(s_0_2)
        let s_0_3: u32 = SecurityStateAtEL(state, tracer, s_0_2);
        // D s_0_4: write-var security <= s_0_3
        fn_state.security = s_0_3;
        // C s_0_5: const #0u : u8
        let s_0_5: bool = false;
        // D s_0_6: write-var InD <= s_0_5
        fn_state.InD = s_0_5;
        // C s_0_7: const #0u : u8
        let s_0_7: bool = false;
        // D s_0_8: write-var InSM <= s_0_7
        fn_state.InSM = s_0_7;
        // D s_0_9: read-var security:u32
        let s_0_9: u32 = fn_state.security;
        // D s_0_10: call PARTIDspaceFromSS(s_0_9)
        let s_0_10: u32 = PARTIDspaceFromSS(state, tracer, s_0_9);
        // D s_0_11: write-var pspace <= s_0_10
        fn_state.pspace = s_0_10;
        // D s_0_12: read-var pspace:u32
        let s_0_12: u32 = fn_state.pspace;
        // C s_0_13: const #3u : u32
        let s_0_13: u32 = 3;
        // D s_0_14: cmp-eq s_0_12 s_0_13
        let s_0_14: bool = ((s_0_12) == (s_0_13));
        // N s_0_15: branch s_0_14 b80 b1
        if s_0_14 {
            return block_80(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#7124 <= s_1_0
        fn_state.gs_7124 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_2_0: read-var gs#7124:u8
        let s_2_0: bool = fn_state.gs_7124;
        // N s_2_1: branch s_2_0 b79 b3
        if s_2_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call UsingAArch32(s_3_0)
        let s_3_1: bool = UsingAArch32(state, tracer, s_3_0);
        // N s_3_2: branch s_3_1 b78 b4
        if s_3_1 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_4_0: read-var acctype:u32
        let s_4_0: u32 = fn_state.acctype;
        // C s_4_1: const #9u : u32
        let s_4_1: u32 = 9;
        // D s_4_2: cmp-eq s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) == (s_4_1));
        // N s_4_3: branch s_4_2 b77 b5
        if s_4_2 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_5_0: read-var el:u8
        let s_5_0: u8 = fn_state.el;
        // D s_5_1: write-var mpamEL <= s_5_0
        fn_state.mpamEL = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // D s_6_1: write-var validEL_name <= s_6_0
        fn_state.validEL_name = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_7_0: const #0u : u32
        let s_7_0: u32 = 0;
        // D s_7_1: read-var acctype:u32
        let s_7_1: u32 = fn_state.acctype;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // D s_7_3: not s_7_2
        let s_7_3: bool = !s_7_2;
        // N s_7_4: branch s_7_3 b47 b8
        if s_7_3 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // D s_8_1: write-var InD <= s_8_0
        fn_state.InD = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_9_0: read-var validEL_name:u8
        let s_9_0: bool = fn_state.validEL_name;
        // D s_9_1: not s_9_0
        let s_9_1: bool = !s_9_0;
        // N s_9_2: branch s_9_1 b46 b10
        if s_9_1 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call HaveRME(s_10_0)
        let s_10_1: bool = HaveRME(state, tracer, s_10_0);
        // N s_10_2: branch s_10_1 b45 b11
        if s_10_1 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#7149 <= s_11_0
        fn_state.gs_7149 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_12_0: read-var gs#7149:u8
        let s_12_0: bool = fn_state.gs_7149;
        // N s_12_1: branch s_12_0 b44 b13
        if s_12_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call HaveMPAMv0p1Ext(s_14_0)
        let s_14_1: bool = HaveMPAMv0p1Ext(state, tracer, s_14_0);
        // N s_14_2: branch s_14_1 b43 b15
        if s_14_1 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#7139 <= s_15_0
        fn_state.gs_7139 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_16_0: read-var gs#7139:u8
        let s_16_0: bool = fn_state.gs_7139;
        // N s_16_1: branch s_16_0 b36 b17
        if s_16_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // N s_17_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call HaveMPAMv0p1Ext(s_18_0)
        let s_18_1: bool = HaveMPAMv0p1Ext(state, tracer, s_18_0);
        // N s_18_2: branch s_18_1 b35 b19
        if s_18_1 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call HaveMPAMv1p1Ext(s_19_0)
        let s_19_1: bool = HaveMPAMv1p1Ext(state, tracer, s_19_0);
        // D s_19_2: write-var gs#7140 <= s_19_1
        fn_state.gs_7140 = s_19_1;
        // N s_19_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_20_0: read-var gs#7140:u8
        let s_20_0: bool = fn_state.gs_7140;
        // N s_20_1: branch s_20_0 b34 b21
        if s_20_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#7141 <= s_21_0
        fn_state.gs_7141 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_22_0: read-var gs#7141:u8
        let s_22_0: bool = fn_state.gs_7141;
        // N s_22_1: branch s_22_0 b28 b23
        if s_22_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // N s_23_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call MPAMisEnabled(s_24_0)
        let s_24_1: bool = MPAMisEnabled(state, tracer, s_24_0);
        // S s_24_2: not s_24_1
        let s_24_2: bool = !s_24_1;
        // N s_24_3: branch s_24_2 b27 b25
        if s_24_2 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_25_0: read-var mpamEL:u8
        let s_25_0: u8 = fn_state.mpamEL;
        // D s_25_1: read-var InD:u8
        let s_25_1: bool = fn_state.InD;
        // D s_25_2: read-var InSM:u8
        let s_25_2: bool = fn_state.InSM;
        // D s_25_3: read-var pspace:u32
        let s_25_3: u32 = fn_state.pspace;
        // D s_25_4: call genMPAM(s_25_0, s_25_1, s_25_2, s_25_3)
        let s_25_4: ProductTypee79b4310dbe79c8c = genMPAM(
            state,
            tracer,
            s_25_0,
            s_25_1,
            s_25_2,
            s_25_3,
        );
        // D s_25_5: write-var return_value <= s_25_4
        fn_state.return_value = s_25_4;
        // N s_25_6: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_26_0: read-var return_value:struct
        let s_26_0: ProductTypee79b4310dbe79c8c = fn_state.return_value;
        // N s_26_1: return s_26_0
        return s_26_0;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_27_0: read-var pspace:u32
        let s_27_0: u32 = fn_state.pspace;
        // D s_27_1: call DefaultMPAMinfo(s_27_0)
        let s_27_1: ProductTypee79b4310dbe79c8c = DefaultMPAMinfo(state, tracer, s_27_0);
        // D s_27_2: write-var return_value <= s_27_1
        fn_state.return_value = s_27_1;
        // N s_27_3: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call MPAM3_EL3_read(s_28_0)
        let s_28_1: ProductType5c790c8ef59cc8b2 = MPAM3_EL3_read(state, tracer, s_28_0);
        // S s_28_2: call _get_MPAM3_EL3_Type_SDEFLT(s_28_1)
        let s_28_2: bool = u_get_MPAM3_EL3_Type_SDEFLT(state, tracer, s_28_1);
        // S s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // C s_28_4: const #1u : u8
        let s_28_4: bool = true;
        // C s_28_5: cast zx s_28_4 -> bv
        let s_28_5: Bits = Bits::new(s_28_4 as u128, 1u16);
        // S s_28_6: cmp-eq s_28_3 s_28_5
        let s_28_6: bool = ((s_28_3) == (s_28_5));
        // N s_28_7: branch s_28_6 b33 b29
        if s_28_6 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#7143 <= s_29_0
        fn_state.gs_7143 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_30_0: read-var gs#7143:u8
        let s_30_0: bool = fn_state.gs_7143;
        // N s_30_1: branch s_30_0 b32 b31
        if s_30_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // N s_31_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_32_0: read-var pspace:u32
        let s_32_0: u32 = fn_state.pspace;
        // D s_32_1: call DefaultMPAMinfo(s_32_0)
        let s_32_1: ProductTypee79b4310dbe79c8c = DefaultMPAMinfo(state, tracer, s_32_0);
        // D s_32_2: write-var return_value <= s_32_1
        fn_state.return_value = s_32_1;
        // N s_32_3: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_33_0: read-var security:u32
        let s_33_0: u32 = fn_state.security;
        // C s_33_1: const #3u : u32
        let s_33_1: u32 = 3;
        // D s_33_2: cmp-eq s_33_0 s_33_1
        let s_33_2: bool = ((s_33_0) == (s_33_1));
        // D s_33_3: write-var gs#7143 <= s_33_2
        fn_state.gs_7143 = s_33_2;
        // N s_33_4: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_34_0: const #11032u : u32
        let s_34_0: u32 = 11032;
        // D s_34_1: read-reg s_34_0:struct
        let s_34_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_34_0 as isize);
            tracer.read_register(s_34_0 as isize, value);
            value
        };
        // D s_34_2: call _get_MPAMIDR_EL1_Type_HAS_SDEFLT(s_34_1)
        let s_34_2: bool = u_get_MPAMIDR_EL1_Type_HAS_SDEFLT(state, tracer, s_34_1);
        // D s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 1u16);
        // C s_34_4: const #1u : u8
        let s_34_4: bool = true;
        // C s_34_5: cast zx s_34_4 -> bv
        let s_34_5: Bits = Bits::new(s_34_4 as u128, 1u16);
        // D s_34_6: cmp-eq s_34_3 s_34_5
        let s_34_6: bool = ((s_34_3) == (s_34_5));
        // D s_34_7: write-var gs#7141 <= s_34_6
        fn_state.gs_7141 = s_34_6;
        // N s_34_8: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var gs#7140 <= s_35_0
        fn_state.gs_7140 = s_35_0;
        // N s_35_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call MPAM3_EL3_read(s_36_0)
        let s_36_1: ProductType5c790c8ef59cc8b2 = MPAM3_EL3_read(state, tracer, s_36_0);
        // S s_36_2: call _get_MPAM3_EL3_Type_FORCE_NS(s_36_1)
        let s_36_2: bool = u_get_MPAM3_EL3_Type_FORCE_NS(state, tracer, s_36_1);
        // S s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 1u16);
        // C s_36_4: const #1u : u8
        let s_36_4: bool = true;
        // C s_36_5: cast zx s_36_4 -> bv
        let s_36_5: Bits = Bits::new(s_36_4 as u128, 1u16);
        // S s_36_6: cmp-eq s_36_3 s_36_5
        let s_36_6: bool = ((s_36_3) == (s_36_5));
        // N s_36_7: branch s_36_6 b42 b37
        if s_36_6 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#7146 <= s_37_0
        fn_state.gs_7146 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_38_0: read-var gs#7146:u8
        let s_38_0: bool = fn_state.gs_7146;
        // N s_38_1: branch s_38_0 b41 b39
        if s_38_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // N s_39_0: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // N s_40_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_41_0: const #3u : u32
        let s_41_0: u32 = 3;
        // D s_41_1: write-var pspace <= s_41_0
        fn_state.pspace = s_41_0;
        // N s_41_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_42_0: read-var security:u32
        let s_42_0: u32 = fn_state.security;
        // C s_42_1: const #3u : u32
        let s_42_1: u32 = 3;
        // D s_42_2: cmp-eq s_42_0 s_42_1
        let s_42_2: bool = ((s_42_0) == (s_42_1));
        // D s_42_3: write-var gs#7146 <= s_42_2
        fn_state.gs_7146 = s_42_2;
        // N s_42_4: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_43_0: const #11032u : u32
        let s_43_0: u32 = 11032;
        // D s_43_1: read-reg s_43_0:struct
        let s_43_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: call _get_MPAMIDR_EL1_Type_HAS_FORCE_NS(s_43_1)
        let s_43_2: bool = u_get_MPAMIDR_EL1_Type_HAS_FORCE_NS(state, tracer, s_43_1);
        // D s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 1u16);
        // C s_43_4: const #1u : u8
        let s_43_4: bool = true;
        // C s_43_5: cast zx s_43_4 -> bv
        let s_43_5: Bits = Bits::new(s_43_4 as u128, 1u16);
        // D s_43_6: cmp-eq s_43_3 s_43_5
        let s_43_6: bool = ((s_43_3) == (s_43_5));
        // D s_43_7: write-var gs#7139 <= s_43_6
        fn_state.gs_7139 = s_43_6;
        // N s_43_8: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_44_0: read-var mpamEL:u8
        let s_44_0: u8 = fn_state.mpamEL;
        // D s_44_1: read-var security:u32
        let s_44_1: u32 = fn_state.security;
        // D s_44_2: read-var pspace:u32
        let s_44_2: u32 = fn_state.pspace;
        // D s_44_3: call AltPARTIDspace(s_44_0, s_44_1, s_44_2)
        let s_44_3: u32 = AltPARTIDspace(state, tracer, s_44_0, s_44_1, s_44_2);
        // D s_44_4: write-var pspace <= s_44_3
        fn_state.pspace = s_44_3;
        // N s_44_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_45_0: const #11032u : u32
        let s_45_0: u32 = 11032;
        // D s_45_1: read-reg s_45_0:struct
        let s_45_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_45_0 as isize);
            tracer.read_register(s_45_0 as isize, value);
            value
        };
        // D s_45_2: call _get_MPAMIDR_EL1_Type_HAS_ALTSP(s_45_1)
        let s_45_2: bool = u_get_MPAMIDR_EL1_Type_HAS_ALTSP(state, tracer, s_45_1);
        // D s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 1u16);
        // C s_45_4: const #1u : u8
        let s_45_4: bool = true;
        // C s_45_5: cast zx s_45_4 -> bv
        let s_45_5: Bits = Bits::new(s_45_4 as u128, 1u16);
        // D s_45_6: cmp-eq s_45_3 s_45_5
        let s_45_6: bool = ((s_45_3) == (s_45_5));
        // D s_45_7: write-var gs#7149 <= s_45_6
        fn_state.gs_7149 = s_45_6;
        // N s_45_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_46_0: read-var pspace:u32
        let s_46_0: u32 = fn_state.pspace;
        // D s_46_1: call DefaultMPAMinfo(s_46_0)
        let s_46_1: ProductTypee79b4310dbe79c8c = DefaultMPAMinfo(state, tracer, s_46_0);
        // D s_46_2: write-var return_value <= s_46_1
        fn_state.return_value = s_46_1;
        // N s_46_3: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_47_0: const #5u : u32
        let s_47_0: u32 = 5;
        // D s_47_1: read-var acctype:u32
        let s_47_1: u32 = fn_state.acctype;
        // D s_47_2: cmp-eq s_47_0 s_47_1
        let s_47_2: bool = ((s_47_0) == (s_47_1));
        // D s_47_3: not s_47_2
        let s_47_3: bool = !s_47_2;
        // N s_47_4: branch s_47_3 b49 b48
        if s_47_3 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_48_0: const #1u : u8
        let s_48_0: bool = true;
        // D s_48_1: write-var InD <= s_48_0
        fn_state.InD = s_48_0;
        // N s_48_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_49_0: const #4u : u32
        let s_49_0: u32 = 4;
        // D s_49_1: read-var acctype:u32
        let s_49_1: u32 = fn_state.acctype;
        // D s_49_2: cmp-eq s_49_0 s_49_1
        let s_49_2: bool = ((s_49_0) == (s_49_1));
        // D s_49_3: not s_49_2
        let s_49_3: bool = !s_49_2;
        // N s_49_4: branch s_49_3 b54 b50
        if s_49_3 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_50_0: const #"Shared SMCU" : str
        let s_50_0: &'static str = "Shared SMCU";
        // S s_50_1: call __IMPDEF_boolean(s_50_0)
        let s_50_1: bool = u__IMPDEF_boolean(state, tracer, s_50_0);
        // N s_50_2: branch s_50_1 b53 b51
        if s_50_1 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_51_0: const #"MPAMSM_EL1 label precedence" : str
        let s_51_0: &'static str = "MPAMSM_EL1 label precedence";
        // S s_51_1: call __IMPDEF_boolean(s_51_0)
        let s_51_1: bool = u__IMPDEF_boolean(state, tracer, s_51_0);
        // D s_51_2: write-var gs#7128 <= s_51_1
        fn_state.gs_7128 = s_51_1;
        // N s_51_3: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_52_0: read-var gs#7128:u8
        let s_52_0: bool = fn_state.gs_7128;
        // D s_52_1: write-var InSM <= s_52_0
        fn_state.InSM = s_52_0;
        // N s_52_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_53_0: const #1u : u8
        let s_53_0: bool = true;
        // D s_53_1: write-var gs#7128 <= s_53_0
        fn_state.gs_7128 = s_53_0;
        // N s_53_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_54_0: const #2u : u32
        let s_54_0: u32 = 2;
        // D s_54_1: read-var acctype:u32
        let s_54_1: u32 = fn_state.acctype;
        // D s_54_2: cmp-eq s_54_0 s_54_1
        let s_54_2: bool = ((s_54_0) == (s_54_1));
        // D s_54_3: not s_54_2
        let s_54_3: bool = !s_54_2;
        // N s_54_4: branch s_54_3 b65 b55
        if s_54_3 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_55_0: const #() : ()
        let s_55_0: () = ();
        // S s_55_1: call HaveSME(s_55_0)
        let s_55_1: bool = HaveSME(state, tracer, s_55_0);
        // N s_55_2: branch s_55_1 b64 b56
        if s_55_1 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // D s_56_1: write-var gs#7130 <= s_56_0
        fn_state.gs_7130 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_57_0: read-var gs#7130:u8
        let s_57_0: bool = fn_state.gs_7130;
        // N s_57_1: branch s_57_0 b60 b58
        if s_57_0 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_58_0: const #0u : u8
        let s_58_0: bool = false;
        // D s_58_1: write-var gs#7132 <= s_58_0
        fn_state.gs_7132 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_59_0: read-var gs#7132:u8
        let s_59_0: bool = fn_state.gs_7132;
        // D s_59_1: write-var InSM <= s_59_0
        fn_state.InSM = s_59_0;
        // N s_59_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_60_0: const #"Shared SMCU" : str
        let s_60_0: &'static str = "Shared SMCU";
        // S s_60_1: call __IMPDEF_boolean(s_60_0)
        let s_60_1: bool = u__IMPDEF_boolean(state, tracer, s_60_0);
        // N s_60_2: branch s_60_1 b63 b61
        if s_60_1 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_61_0: const #"MPAMSM_EL1 label precedence" : str
        let s_61_0: &'static str = "MPAMSM_EL1 label precedence";
        // S s_61_1: call __IMPDEF_boolean(s_61_0)
        let s_61_1: bool = u__IMPDEF_boolean(state, tracer, s_61_0);
        // D s_61_2: write-var gs#7131 <= s_61_1
        fn_state.gs_7131 = s_61_1;
        // N s_61_3: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_62_0: read-var gs#7131:u8
        let s_62_0: bool = fn_state.gs_7131;
        // D s_62_1: write-var gs#7132 <= s_62_0
        fn_state.gs_7132 = s_62_0;
        // N s_62_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_63_0: const #1u : u8
        let s_63_0: bool = true;
        // D s_63_1: write-var gs#7131 <= s_63_0
        fn_state.gs_7131 = s_63_0;
        // N s_63_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_64_0: const #16989u : u32
        let s_64_0: u32 = 16989;
        // D s_64_1: read-reg s_64_0:u8
        let s_64_1: bool = {
            let value = state.read_register::<bool>(s_64_0 as isize);
            tracer.read_register(s_64_0 as isize, value);
            value
        };
        // D s_64_2: cast zx s_64_1 -> bv
        let s_64_2: Bits = Bits::new(s_64_1 as u128, 1u16);
        // C s_64_3: const #1u : u8
        let s_64_3: bool = true;
        // C s_64_4: cast zx s_64_3 -> bv
        let s_64_4: Bits = Bits::new(s_64_3 as u128, 1u16);
        // D s_64_5: cmp-eq s_64_2 s_64_4
        let s_64_5: bool = ((s_64_2) == (s_64_4));
        // D s_64_6: write-var gs#7130 <= s_64_5
        fn_state.gs_7130 = s_64_5;
        // N s_64_7: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_65_0: const #3u : u32
        let s_65_0: u32 = 3;
        // D s_65_1: read-var acctype:u32
        let s_65_1: u32 = fn_state.acctype;
        // D s_65_2: cmp-eq s_65_0 s_65_1
        let s_65_2: bool = ((s_65_0) == (s_65_1));
        // D s_65_3: not s_65_2
        let s_65_3: bool = !s_65_2;
        // N s_65_4: branch s_65_3 b76 b66
        if s_65_3 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_66_0: const #() : ()
        let s_66_0: () = ();
        // S s_66_1: call HaveSME(s_66_0)
        let s_66_1: bool = HaveSME(state, tracer, s_66_0);
        // N s_66_2: branch s_66_1 b75 b67
        if s_66_1 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_67_0: const #0u : u8
        let s_67_0: bool = false;
        // D s_67_1: write-var gs#7134 <= s_67_0
        fn_state.gs_7134 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_68_0: read-var gs#7134:u8
        let s_68_0: bool = fn_state.gs_7134;
        // N s_68_1: branch s_68_0 b71 b69
        if s_68_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_69_0: const #0u : u8
        let s_69_0: bool = false;
        // D s_69_1: write-var gs#7136 <= s_69_0
        fn_state.gs_7136 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_70_0: read-var gs#7136:u8
        let s_70_0: bool = fn_state.gs_7136;
        // D s_70_1: write-var InSM <= s_70_0
        fn_state.InSM = s_70_0;
        // N s_70_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_71_0: const #"Shared SMCU" : str
        let s_71_0: &'static str = "Shared SMCU";
        // S s_71_1: call __IMPDEF_boolean(s_71_0)
        let s_71_1: bool = u__IMPDEF_boolean(state, tracer, s_71_0);
        // N s_71_2: branch s_71_1 b74 b72
        if s_71_1 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_72_0: const #"MPAMSM_EL1 label precedence" : str
        let s_72_0: &'static str = "MPAMSM_EL1 label precedence";
        // S s_72_1: call __IMPDEF_boolean(s_72_0)
        let s_72_1: bool = u__IMPDEF_boolean(state, tracer, s_72_0);
        // D s_72_2: write-var gs#7135 <= s_72_1
        fn_state.gs_7135 = s_72_1;
        // N s_72_3: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_73_0: read-var gs#7135:u8
        let s_73_0: bool = fn_state.gs_7135;
        // D s_73_1: write-var gs#7136 <= s_73_0
        fn_state.gs_7136 = s_73_0;
        // N s_73_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_74_0: const #1u : u8
        let s_74_0: bool = true;
        // D s_74_1: write-var gs#7135 <= s_74_0
        fn_state.gs_7135 = s_74_0;
        // N s_74_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_75_0: const #16989u : u32
        let s_75_0: u32 = 16989;
        // D s_75_1: read-reg s_75_0:u8
        let s_75_1: bool = {
            let value = state.read_register::<bool>(s_75_0 as isize);
            tracer.read_register(s_75_0 as isize, value);
            value
        };
        // D s_75_2: cast zx s_75_1 -> bv
        let s_75_2: Bits = Bits::new(s_75_1 as u128, 1u16);
        // C s_75_3: const #1u : u8
        let s_75_3: bool = true;
        // C s_75_4: cast zx s_75_3 -> bv
        let s_75_4: Bits = Bits::new(s_75_3 as u128, 1u16);
        // D s_75_5: cmp-eq s_75_2 s_75_4
        let s_75_5: bool = ((s_75_2) == (s_75_4));
        // D s_75_6: write-var gs#7134 <= s_75_5
        fn_state.gs_7134 = s_75_5;
        // N s_75_7: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_76_0: const #0u : u8
        let s_76_0: bool = false;
        // D s_76_1: write-var InD <= s_76_0
        fn_state.InD = s_76_0;
        // N s_76_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_77_0: const #432u : u32
        let s_77_0: u32 = 432;
        // D s_77_1: read-reg s_77_0:u8
        let s_77_1: u8 = {
            let value = state.read_register::<u8>(s_77_0 as isize);
            tracer.read_register(s_77_0 as isize, value);
            value
        };
        // D s_77_2: write-var mpamEL <= s_77_1
        fn_state.mpamEL = s_77_1;
        // N s_77_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_78_0: const #16983u : u32
        let s_78_0: u32 = 16983;
        // D s_78_1: read-reg s_78_0:u8
        let s_78_1: u8 = {
            let value = state.read_register::<u8>(s_78_0 as isize);
            tracer.read_register(s_78_0 as isize, value);
            value
        };
        // D s_78_2: call ELFromM32(s_78_1)
        let s_78_2: ProductTypea5cc8de4daab131c = ELFromM32(state, tracer, s_78_1);
        // D s_78_3: write-var ga#4855 <= s_78_2
        fn_state.ga_4855 = s_78_2;
        // D s_78_4: read-var ga#4855.0:struct
        let s_78_4: bool = fn_state.ga_4855._0;
        // D s_78_5: read-var ga#4855.1:struct
        let s_78_5: u8 = fn_state.ga_4855._1;
        // D s_78_6: write-var validEL_name <= s_78_4
        fn_state.validEL_name = s_78_4;
        // D s_78_7: write-var mpamEL <= s_78_5
        fn_state.mpamEL = s_78_5;
        // N s_78_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_79_0: read-var pspace:u32
        let s_79_0: u32 = fn_state.pspace;
        // D s_79_1: call DefaultMPAMinfo(s_79_0)
        let s_79_1: ProductTypee79b4310dbe79c8c = DefaultMPAMinfo(state, tracer, s_79_0);
        // D s_79_2: write-var return_value <= s_79_1
        fn_state.return_value = s_79_1;
        // N s_79_3: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_80_0: const #() : ()
        let s_80_0: () = ();
        // S s_80_1: call MPAMisEnabled(s_80_0)
        let s_80_1: bool = MPAMisEnabled(state, tracer, s_80_0);
        // S s_80_2: not s_80_1
        let s_80_2: bool = !s_80_1;
        // D s_80_3: write-var gs#7124 <= s_80_2
        fn_state.gs_7124 = s_80_2;
        // N s_80_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
