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
use u_get_SCR_Type_TWE::*;
use HCR_read::*;
use AArch32_TakeUndefInstrException::*;
use ConditionSyndrome::*;
use u_get_HCR_Type_TWE::*;
use SCTLR_read__2::*;
use u_get_SCR_Type_TWI::*;
use u_get_HCR_Type_TWI::*;
use ELUsingAArch32::*;
use ExceptionSyndrome::*;
use AArch32_TakeHypTrapException__1::*;
use AArch64_CheckForWFxTrap::*;
use u_get_SCTLR_Type_nTWI::*;
use Bit::*;
use AArch64_WFxTrap::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_SCTLR_Type_nTWE::*;
use AArch32_TakeMonitorTrapException::*;
use EL2Enabled::*;
use common::*;
pub fn AArch32_CheckForWFxTrap<T: Tracer>(
    state: &mut State,
    tracer: &T,
    target_el: u8,
    wfxtype: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        trap: bool,
        ga_24892: bool,
        except: ProductTypeb7f99f96751e17c4,
        ga_24893: bool,
        gs_31887: bool,
        is_wfe: bool,
        ga_24889: bool,
        gs_31886: bool,
        gs_31888: bool,
        target_el: u8,
        wfxtype: u32,
    }
    let fn_state = FunctionState {
        target_el,
        wfxtype,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var target_el:u8
        let s_0_0: u8 = fn_state.target_el;
        // C s_0_1: const #2u : u8
        let s_0_1: u8 = 2;
        // D s_0_2: cmp-lt s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) < (s_0_1));
        // N s_0_3: assert s_0_2
        let s_0_3: () = assert!(s_0_2);
        // D s_0_4: read-var target_el:u8
        let s_0_4: u8 = fn_state.target_el;
        // D s_0_5: call ELUsingAArch32(s_0_4)
        let s_0_5: bool = ELUsingAArch32(state, tracer, s_0_4);
        // D s_0_6: not s_0_5
        let s_0_6: bool = !s_0_5;
        // N s_0_7: branch s_0_6 b41 b1
        if s_0_6 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var wfxtype:u32
        let s_1_0: u32 = fn_state.wfxtype;
        // C s_1_1: const #0u : u32
        let s_1_1: u32 = 0;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // D s_1_3: write-var is_wfe <= s_1_2
        fn_state.is_wfe = s_1_2;
        // D s_1_4: read-var target_el:u8
        let s_1_4: u8 = fn_state.target_el;
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // C s_1_6: const #440u : u32
        let s_1_6: u32 = 440;
        // D s_1_7: read-reg s_1_6:u8
        let s_1_7: u8 = {
            let value = state.read_register::<u8>(s_1_6 as isize);
            tracer.read_register(s_1_6 as isize, value);
            value
        };
        // D s_1_8: cast zx s_1_7 -> bv
        let s_1_8: Bits = Bits::new(s_1_7 as u128, 2u16);
        // D s_1_9: cmp-eq s_1_5 s_1_8
        let s_1_9: bool = ((s_1_5) == (s_1_8));
        // D s_1_10: not s_1_9
        let s_1_10: bool = !s_1_9;
        // N s_1_11: branch s_1_10 b30 b2
        if s_1_10 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var is_wfe:u8
        let s_2_0: bool = fn_state.is_wfe;
        // N s_2_1: branch s_2_0 b29 b3
        if s_2_0 {
            return block_29(state, tracer, fn_state);
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
        // S s_3_1: call SCTLR_read__2(s_3_0)
        let s_3_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_3_0);
        // S s_3_2: call _get_SCTLR_Type_nTWI(s_3_1)
        let s_3_2: bool = u_get_SCTLR_Type_nTWI(state, tracer, s_3_1);
        // D s_3_3: write-var ga#24889 <= s_3_2
        fn_state.ga_24889 = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#24889:u8
        let s_4_0: bool = fn_state.ga_24889;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 1u16);
        // C s_4_2: const #0u : u8
        let s_4_2: bool = false;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // D s_4_5: write-var trap <= s_4_4
        fn_state.trap = s_4_4;
        // N s_4_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var trap:u8
        let s_5_0: bool = fn_state.trap;
        // N s_5_1: branch s_5_0 b7 b6
        if s_5_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var target_el:u8
        let s_7_0: u8 = fn_state.target_el;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 2u16);
        // C s_7_2: const #440u : u32
        let s_7_2: u32 = 440;
        // D s_7_3: read-reg s_7_2:u8
        let s_7_3: u8 = {
            let value = state.read_register::<u8>(s_7_2 as isize);
            tracer.read_register(s_7_2 as isize, value);
            value
        };
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 2u16);
        // D s_7_5: cmp-eq s_7_1 s_7_4
        let s_7_5: bool = ((s_7_1) == (s_7_4));
        // N s_7_6: branch s_7_5 b28 b8
        if s_7_5 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#31886 <= s_8_0
        fn_state.gs_31886 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#31886:u8
        let s_9_0: bool = fn_state.gs_31886;
        // N s_9_1: branch s_9_0 b27 b10
        if s_9_0 {
            return block_27(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#31887 <= s_10_0
        fn_state.gs_31887 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#31887:u8
        let s_11_0: bool = fn_state.gs_31887;
        // N s_11_1: branch s_11_0 b26 b12
        if s_11_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#31888 <= s_12_0
        fn_state.gs_31888 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#31888:u8
        let s_13_0: bool = fn_state.gs_31888;
        // N s_13_1: branch s_13_0 b25 b14
        if s_13_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var target_el:u8
        let s_15_0: u8 = fn_state.target_el;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 2u16);
        // C s_15_2: const #424u : u32
        let s_15_2: u32 = 424;
        // D s_15_3: read-reg s_15_2:u8
        let s_15_3: u8 = {
            let value = state.read_register::<u8>(s_15_2 as isize);
            tracer.read_register(s_15_2 as isize, value);
            value
        };
        // D s_15_4: cast zx s_15_3 -> bv
        let s_15_4: Bits = Bits::new(s_15_3 as u128, 2u16);
        // D s_15_5: cmp-eq s_15_1 s_15_4
        let s_15_5: bool = ((s_15_1) == (s_15_4));
        // N s_15_6: branch s_15_5 b24 b16
        if s_15_5 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var target_el:u8
        let s_16_0: u8 = fn_state.target_el;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 2u16);
        // C s_16_2: const #432u : u32
        let s_16_2: u32 = 432;
        // D s_16_3: read-reg s_16_2:u8
        let s_16_3: u8 = {
            let value = state.read_register::<u8>(s_16_2 as isize);
            tracer.read_register(s_16_2 as isize, value);
            value
        };
        // D s_16_4: cast zx s_16_3 -> bv
        let s_16_4: Bits = Bits::new(s_16_3 as u128, 2u16);
        // D s_16_5: cmp-eq s_16_1 s_16_4
        let s_16_5: bool = ((s_16_1) == (s_16_4));
        // N s_16_6: branch s_16_5 b18 b17
        if s_16_5 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call AArch32_TakeUndefInstrException(s_17_0)
        let s_17_1: () = AArch32_TakeUndefInstrException(state, tracer, s_17_0);
        // N s_17_2: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u32
        let s_18_0: u32 = 1;
        // S s_18_1: call ExceptionSyndrome(s_18_0)
        let s_18_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_18_0,
        );
        // D s_18_2: write-var except <= s_18_1
        fn_state.except = s_18_1;
        // C s_18_3: const #() : ()
        let s_18_3: () = ();
        // S s_18_4: call ConditionSyndrome(s_18_3)
        let s_18_4: u8 = ConditionSyndrome(state, tracer, s_18_3);
        // D s_18_5: read-var except:struct
        let s_18_5: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_18_6: write-var except <= s_18_5
        fn_state.except = s_18_5;
        // C s_18_7: const #1u : u32
        let s_18_7: u32 = 1;
        // D s_18_8: read-var wfxtype:u32
        let s_18_8: u32 = fn_state.wfxtype;
        // D s_18_9: cmp-eq s_18_7 s_18_8
        let s_18_9: bool = ((s_18_7) == (s_18_8));
        // D s_18_10: not s_18_9
        let s_18_10: bool = !s_18_9;
        // N s_18_11: branch s_18_10 b21 b19
        if s_18_10 {
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
        // S s_19_1: call Bit(s_19_0)
        let s_19_1: bool = Bit(state, tracer, s_19_0);
        // D s_19_2: read-var except:struct
        let s_19_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_19_3: write-var except <= s_19_2
        fn_state.except = s_19_2;
        // N s_19_4: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var except:struct
        let s_20_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_20_1: call AArch32_TakeHypTrapException__1(s_20_0)
        let s_20_1: () = AArch32_TakeHypTrapException__1(state, tracer, s_20_0);
        // N s_20_2: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u32
        let s_21_0: u32 = 0;
        // D s_21_1: read-var wfxtype:u32
        let s_21_1: u32 = fn_state.wfxtype;
        // D s_21_2: cmp-eq s_21_0 s_21_1
        let s_21_2: bool = ((s_21_0) == (s_21_1));
        // D s_21_3: not s_21_2
        let s_21_3: bool = !s_21_2;
        // N s_21_4: branch s_21_3 b23 b22
        if s_21_3 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // S s_22_1: call Bit(s_22_0)
        let s_22_1: bool = Bit(state, tracer, s_22_0);
        // D s_22_2: read-var except:struct
        let s_22_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_22_3: write-var except <= s_22_2
        fn_state.except = s_22_2;
        // N s_22_4: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call AArch32_TakeMonitorTrapException(s_24_0)
        let s_24_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_24_0);
        // N s_24_2: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var wfxtype:u32
        let s_25_0: u32 = fn_state.wfxtype;
        // D s_25_1: read-var target_el:u8
        let s_25_1: u8 = fn_state.target_el;
        // D s_25_2: call AArch64_WFxTrap(s_25_0, s_25_1)
        let s_25_2: () = AArch64_WFxTrap(state, tracer, s_25_0, s_25_1);
        // N s_25_3: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #102552u : u32
        let s_26_0: u32 = 102552;
        // D s_26_1: read-reg s_26_0:struct
        let s_26_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: call _get_HCR_EL2_Type_TGE(s_26_1)
        let s_26_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_26_1);
        // D s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // C s_26_4: const #1u : u8
        let s_26_4: bool = true;
        // C s_26_5: cast zx s_26_4 -> bv
        let s_26_5: Bits = Bits::new(s_26_4 as u128, 1u16);
        // D s_26_6: cmp-eq s_26_3 s_26_5
        let s_26_6: bool = ((s_26_3) == (s_26_5));
        // D s_26_7: write-var gs#31888 <= s_26_6
        fn_state.gs_31888 = s_26_6;
        // N s_26_8: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #432u : u32
        let s_27_0: u32 = 432;
        // D s_27_1: read-reg s_27_0:u8
        let s_27_1: u8 = {
            let value = state.read_register::<u8>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: call ELUsingAArch32(s_27_1)
        let s_27_2: bool = ELUsingAArch32(state, tracer, s_27_1);
        // D s_27_3: not s_27_2
        let s_27_3: bool = !s_27_2;
        // D s_27_4: write-var gs#31887 <= s_27_3
        fn_state.gs_31887 = s_27_3;
        // N s_27_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call EL2Enabled(s_28_0)
        let s_28_1: bool = EL2Enabled(state, tracer, s_28_0);
        // D s_28_2: write-var gs#31886 <= s_28_1
        fn_state.gs_31886 = s_28_1;
        // N s_28_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call SCTLR_read__2(s_29_0)
        let s_29_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_29_0);
        // S s_29_2: call _get_SCTLR_Type_nTWE(s_29_1)
        let s_29_2: bool = u_get_SCTLR_Type_nTWE(state, tracer, s_29_1);
        // D s_29_3: write-var ga#24889 <= s_29_2
        fn_state.ga_24889 = s_29_2;
        // N s_29_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var target_el:u8
        let s_30_0: u8 = fn_state.target_el;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 2u16);
        // C s_30_2: const #432u : u32
        let s_30_2: u32 = 432;
        // D s_30_3: read-reg s_30_2:u8
        let s_30_3: u8 = {
            let value = state.read_register::<u8>(s_30_2 as isize);
            tracer.read_register(s_30_2 as isize, value);
            value
        };
        // D s_30_4: cast zx s_30_3 -> bv
        let s_30_4: Bits = Bits::new(s_30_3 as u128, 2u16);
        // D s_30_5: cmp-eq s_30_1 s_30_4
        let s_30_5: bool = ((s_30_1) == (s_30_4));
        // D s_30_6: not s_30_5
        let s_30_6: bool = !s_30_5;
        // N s_30_7: branch s_30_6 b35 b31
        if s_30_6 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var is_wfe:u8
        let s_31_0: bool = fn_state.is_wfe;
        // N s_31_1: branch s_31_0 b34 b32
        if s_31_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call HCR_read(s_32_0)
        let s_32_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_32_0);
        // S s_32_2: call _get_HCR_Type_TWI(s_32_1)
        let s_32_2: bool = u_get_HCR_Type_TWI(state, tracer, s_32_1);
        // D s_32_3: write-var ga#24892 <= s_32_2
        fn_state.ga_24892 = s_32_2;
        // N s_32_4: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var ga#24892:u8
        let s_33_0: bool = fn_state.ga_24892;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // C s_33_2: const #1u : u8
        let s_33_2: bool = true;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: write-var trap <= s_33_4
        fn_state.trap = s_33_4;
        // N s_33_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #() : ()
        let s_34_0: () = ();
        // S s_34_1: call HCR_read(s_34_0)
        let s_34_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_34_0);
        // S s_34_2: call _get_HCR_Type_TWE(s_34_1)
        let s_34_2: bool = u_get_HCR_Type_TWE(state, tracer, s_34_1);
        // D s_34_3: write-var ga#24892 <= s_34_2
        fn_state.ga_24892 = s_34_2;
        // N s_34_4: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var target_el:u8
        let s_35_0: u8 = fn_state.target_el;
        // D s_35_1: cast zx s_35_0 -> bv
        let s_35_1: Bits = Bits::new(s_35_0 as u128, 2u16);
        // C s_35_2: const #424u : u32
        let s_35_2: u32 = 424;
        // D s_35_3: read-reg s_35_2:u8
        let s_35_3: u8 = {
            let value = state.read_register::<u8>(s_35_2 as isize);
            tracer.read_register(s_35_2 as isize, value);
            value
        };
        // D s_35_4: cast zx s_35_3 -> bv
        let s_35_4: Bits = Bits::new(s_35_3 as u128, 2u16);
        // D s_35_5: cmp-eq s_35_1 s_35_4
        let s_35_5: bool = ((s_35_1) == (s_35_4));
        // D s_35_6: not s_35_5
        let s_35_6: bool = !s_35_5;
        // N s_35_7: branch s_35_6 b40 b36
        if s_35_6 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var is_wfe:u8
        let s_36_0: bool = fn_state.is_wfe;
        // N s_36_1: branch s_36_0 b39 b37
        if s_36_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #20920u : u32
        let s_37_0: u32 = 20920;
        // D s_37_1: read-reg s_37_0:struct
        let s_37_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_37_0 as isize);
            tracer.read_register(s_37_0 as isize, value);
            value
        };
        // D s_37_2: call _get_SCR_Type_TWI(s_37_1)
        let s_37_2: bool = u_get_SCR_Type_TWI(state, tracer, s_37_1);
        // D s_37_3: write-var ga#24893 <= s_37_2
        fn_state.ga_24893 = s_37_2;
        // N s_37_4: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var ga#24893:u8
        let s_38_0: bool = fn_state.ga_24893;
        // D s_38_1: cast zx s_38_0 -> bv
        let s_38_1: Bits = Bits::new(s_38_0 as u128, 1u16);
        // C s_38_2: const #1u : u8
        let s_38_2: bool = true;
        // C s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 1u16);
        // D s_38_4: cmp-eq s_38_1 s_38_3
        let s_38_4: bool = ((s_38_1) == (s_38_3));
        // D s_38_5: write-var trap <= s_38_4
        fn_state.trap = s_38_4;
        // N s_38_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #20920u : u32
        let s_39_0: u32 = 20920;
        // D s_39_1: read-reg s_39_0:struct
        let s_39_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // D s_39_2: call _get_SCR_Type_TWE(s_39_1)
        let s_39_2: bool = u_get_SCR_Type_TWE(state, tracer, s_39_1);
        // D s_39_3: write-var ga#24893 <= s_39_2
        fn_state.ga_24893 = s_39_2;
        // N s_39_4: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_40_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var target_el:u8
        let s_41_0: u8 = fn_state.target_el;
        // D s_41_1: read-var wfxtype:u32
        let s_41_1: u32 = fn_state.wfxtype;
        // D s_41_2: call AArch64_CheckForWFxTrap(s_41_0, s_41_1)
        let s_41_2: () = AArch64_CheckForWFxTrap(state, tracer, s_41_0, s_41_1);
        // N s_41_3: return
        return;
    }
}
