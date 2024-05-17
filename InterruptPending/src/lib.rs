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
use u_get_HCR_EL2_Type_IMO::*;
use IsVirtualSErrorPending::*;
use VirtualFIQPending::*;
use u_get_HCR_EL2_Type_VF::*;
use VirtualIRQPending::*;
use u_get_HCR_EL2_Type_VI::*;
use IRQPending::*;
use u_get_HCR_EL2_Type_VSE::*;
use u_get_HCR_EL2_Type_TGE::*;
use IsPhysicalSErrorPending::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_AMO::*;
use FIQPending::*;
use u_get_HCR_EL2_Type_FMO::*;
use common::*;
pub fn InterruptPending<T: Tracer>(state: &mut State, tracer: &T, gs_6677: ()) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_6687: bool,
        gs_6684: bool,
        gs_6685: bool,
        ga_4472: ProductType8b847afc727d5818,
        pending_physical_interrupt: bool,
        pending_virtual_interrupt: bool,
        gs_6696: bool,
        ga_4471: ProductType8b847afc727d5818,
        gs_6697: bool,
        gs_6691: bool,
        gs_6695: bool,
        gs_6694: bool,
        fiq_pending: bool,
        gs_6686: bool,
        virq_pending: bool,
        gs_6698: bool,
        gs_6692: bool,
        vfiq_pending: bool,
        gs_6690: bool,
        gs_6693: bool,
        gs_6688: bool,
        gs_6677: (),
    }
    let fn_state = FunctionState {
        gs_6677,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #0u : u8
        let s_0_0: bool = false;
        // D s_0_1: write-var pending_virtual_interrupt <= s_0_0
        fn_state.pending_virtual_interrupt = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call IRQPending(s_0_2)
        let s_0_3: ProductType8b847afc727d5818 = IRQPending(state, tracer, s_0_2);
        // D s_0_4: write-var ga#4471 <= s_0_3
        fn_state.ga_4471 = s_0_3;
        // D s_0_5: read-var ga#4471.0:struct
        let s_0_5: bool = fn_state.ga_4471._0;
        // C s_0_6: const #() : ()
        let s_0_6: () = ();
        // S s_0_7: call FIQPending(s_0_6)
        let s_0_7: ProductType8b847afc727d5818 = FIQPending(state, tracer, s_0_6);
        // D s_0_8: write-var ga#4472 <= s_0_7
        fn_state.ga_4472 = s_0_7;
        // D s_0_9: read-var ga#4472.0:struct
        let s_0_9: bool = fn_state.ga_4472._0;
        // D s_0_10: write-var fiq_pending <= s_0_9
        fn_state.fiq_pending = s_0_9;
        // N s_0_11: branch s_0_5 b45 b1
        if s_0_5 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_1_0: read-var fiq_pending:u8
        let s_1_0: bool = fn_state.fiq_pending;
        // D s_1_1: write-var gs#6684 <= s_1_0
        fn_state.gs_6684 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#6684:u8
        let s_2_0: bool = fn_state.gs_6684;
        // N s_2_1: branch s_2_0 b44 b3
        if s_2_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call IsPhysicalSErrorPending(s_3_0)
        let s_3_1: bool = IsPhysicalSErrorPending(state, tracer, s_3_0);
        // D s_3_2: write-var gs#6685 <= s_3_1
        fn_state.gs_6685 = s_3_1;
        // N s_3_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var gs#6685:u8
        let s_4_0: bool = fn_state.gs_6685;
        // D s_4_1: write-var pending_physical_interrupt <= s_4_0
        fn_state.pending_physical_interrupt = s_4_0;
        // C s_4_2: const #() : ()
        let s_4_2: () = ();
        // S s_4_3: call EL2Enabled(s_4_2)
        let s_4_3: bool = EL2Enabled(state, tracer, s_4_2);
        // N s_4_4: branch s_4_3 b40 b5
        if s_4_3 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#6687 <= s_5_0
        fn_state.gs_6687 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var gs#6687:u8
        let s_6_0: bool = fn_state.gs_6687;
        // N s_6_1: branch s_6_0 b39 b7
        if s_6_0 {
            return block_39(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#6688 <= s_7_0
        fn_state.gs_6688 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var gs#6688:u8
        let s_8_0: bool = fn_state.gs_6688;
        // N s_8_1: branch s_8_0 b14 b9
        if s_8_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var pending_physical_interrupt:u8
        let s_10_0: bool = fn_state.pending_physical_interrupt;
        // N s_10_1: branch s_10_0 b13 b11
        if s_10_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var pending_virtual_interrupt:u8
        let s_11_0: bool = fn_state.pending_virtual_interrupt;
        // D s_11_1: write-var gs#6698 <= s_11_0
        fn_state.gs_6698 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var gs#6698:u8
        let s_12_0: bool = fn_state.gs_6698;
        // N s_12_1: return s_12_0
        return s_12_0;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#6698 <= s_13_0
        fn_state.gs_6698 = s_13_0;
        // N s_13_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #102552u : u32
        let s_14_0: u32 = 102552;
        // D s_14_1: read-reg s_14_0:struct
        let s_14_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: call _get_HCR_EL2_Type_IMO(s_14_1)
        let s_14_2: bool = u_get_HCR_EL2_Type_IMO(state, tracer, s_14_1);
        // D s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // C s_14_4: const #1u : u8
        let s_14_4: bool = true;
        // C s_14_5: cast zx s_14_4 -> bv
        let s_14_5: Bits = Bits::new(s_14_4 as u128, 1u16);
        // D s_14_6: cmp-eq s_14_3 s_14_5
        let s_14_6: bool = ((s_14_3) == (s_14_5));
        // N s_14_7: branch s_14_6 b35 b15
        if s_14_6 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#6691 <= s_15_0
        fn_state.gs_6691 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_16_0: read-var gs#6691:u8
        let s_16_0: bool = fn_state.gs_6691;
        // D s_16_1: write-var virq_pending <= s_16_0
        fn_state.virq_pending = s_16_0;
        // C s_16_2: const #102552u : u32
        let s_16_2: u32 = 102552;
        // D s_16_3: read-reg s_16_2:struct
        let s_16_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_16_2 as isize);
            tracer.read_register(s_16_2 as isize, value);
            value
        };
        // D s_16_4: call _get_HCR_EL2_Type_FMO(s_16_3)
        let s_16_4: bool = u_get_HCR_EL2_Type_FMO(state, tracer, s_16_3);
        // D s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 1u16);
        // C s_16_6: const #1u : u8
        let s_16_6: bool = true;
        // C s_16_7: cast zx s_16_6 -> bv
        let s_16_7: Bits = Bits::new(s_16_6 as u128, 1u16);
        // D s_16_8: cmp-eq s_16_5 s_16_7
        let s_16_8: bool = ((s_16_5) == (s_16_7));
        // N s_16_9: branch s_16_8 b31 b17
        if s_16_8 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#6693 <= s_17_0
        fn_state.gs_6693 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var gs#6693:u8
        let s_18_0: bool = fn_state.gs_6693;
        // D s_18_1: write-var vfiq_pending <= s_18_0
        fn_state.vfiq_pending = s_18_0;
        // C s_18_2: const #102552u : u32
        let s_18_2: u32 = 102552;
        // D s_18_3: read-reg s_18_2:struct
        let s_18_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_2 as isize);
            tracer.read_register(s_18_2 as isize, value);
            value
        };
        // D s_18_4: call _get_HCR_EL2_Type_AMO(s_18_3)
        let s_18_4: bool = u_get_HCR_EL2_Type_AMO(state, tracer, s_18_3);
        // D s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 1u16);
        // C s_18_6: const #1u : u8
        let s_18_6: bool = true;
        // C s_18_7: cast zx s_18_6 -> bv
        let s_18_7: Bits = Bits::new(s_18_6 as u128, 1u16);
        // D s_18_8: cmp-eq s_18_5 s_18_7
        let s_18_8: bool = ((s_18_5) == (s_18_7));
        // N s_18_9: branch s_18_8 b27 b19
        if s_18_8 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#6695 <= s_19_0
        fn_state.gs_6695 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_20_0: read-var gs#6695:u8
        let s_20_0: bool = fn_state.gs_6695;
        // N s_20_1: branch s_20_0 b26 b21
        if s_20_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_21_0: read-var virq_pending:u8
        let s_21_0: bool = fn_state.virq_pending;
        // D s_21_1: write-var gs#6696 <= s_21_0
        fn_state.gs_6696 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_22_0: read-var gs#6696:u8
        let s_22_0: bool = fn_state.gs_6696;
        // N s_22_1: branch s_22_0 b25 b23
        if s_22_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_23_0: read-var vfiq_pending:u8
        let s_23_0: bool = fn_state.vfiq_pending;
        // D s_23_1: write-var gs#6697 <= s_23_0
        fn_state.gs_6697 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_24_0: read-var gs#6697:u8
        let s_24_0: bool = fn_state.gs_6697;
        // D s_24_1: write-var pending_virtual_interrupt <= s_24_0
        fn_state.pending_virtual_interrupt = s_24_0;
        // N s_24_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#6697 <= s_25_0
        fn_state.gs_6697 = s_25_0;
        // N s_25_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#6696 <= s_26_0
        fn_state.gs_6696 = s_26_0;
        // N s_26_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call IsVirtualSErrorPending(s_27_0)
        let s_27_1: bool = IsVirtualSErrorPending(state, tracer, s_27_0);
        // N s_27_2: branch s_27_1 b30 b28
        if s_27_1 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_28_0: const #102552u : u32
        let s_28_0: u32 = 102552;
        // D s_28_1: read-reg s_28_0:struct
        let s_28_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call _get_HCR_EL2_Type_VSE(s_28_1)
        let s_28_2: bool = u_get_HCR_EL2_Type_VSE(state, tracer, s_28_1);
        // D s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // C s_28_4: const #1u : u8
        let s_28_4: bool = true;
        // C s_28_5: cast zx s_28_4 -> bv
        let s_28_5: Bits = Bits::new(s_28_4 as u128, 1u16);
        // D s_28_6: cmp-eq s_28_3 s_28_5
        let s_28_6: bool = ((s_28_3) == (s_28_5));
        // D s_28_7: write-var gs#6694 <= s_28_6
        fn_state.gs_6694 = s_28_6;
        // N s_28_8: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_29_0: read-var gs#6694:u8
        let s_29_0: bool = fn_state.gs_6694;
        // D s_29_1: write-var gs#6695 <= s_29_0
        fn_state.gs_6695 = s_29_0;
        // N s_29_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var gs#6694 <= s_30_0
        fn_state.gs_6694 = s_30_0;
        // N s_30_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call VirtualFIQPending(s_31_0)
        let s_31_1: bool = VirtualFIQPending(state, tracer, s_31_0);
        // N s_31_2: branch s_31_1 b34 b32
        if s_31_1 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_32_0: const #102552u : u32
        let s_32_0: u32 = 102552;
        // D s_32_1: read-reg s_32_0:struct
        let s_32_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_32_0 as isize);
            tracer.read_register(s_32_0 as isize, value);
            value
        };
        // D s_32_2: call _get_HCR_EL2_Type_VF(s_32_1)
        let s_32_2: bool = u_get_HCR_EL2_Type_VF(state, tracer, s_32_1);
        // D s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // C s_32_4: const #1u : u8
        let s_32_4: bool = true;
        // C s_32_5: cast zx s_32_4 -> bv
        let s_32_5: Bits = Bits::new(s_32_4 as u128, 1u16);
        // D s_32_6: cmp-eq s_32_3 s_32_5
        let s_32_6: bool = ((s_32_3) == (s_32_5));
        // D s_32_7: write-var gs#6692 <= s_32_6
        fn_state.gs_6692 = s_32_6;
        // N s_32_8: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_33_0: read-var gs#6692:u8
        let s_33_0: bool = fn_state.gs_6692;
        // D s_33_1: write-var gs#6693 <= s_33_0
        fn_state.gs_6693 = s_33_0;
        // N s_33_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var gs#6692 <= s_34_0
        fn_state.gs_6692 = s_34_0;
        // N s_34_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_35_0: const #() : ()
        let s_35_0: () = ();
        // S s_35_1: call VirtualIRQPending(s_35_0)
        let s_35_1: bool = VirtualIRQPending(state, tracer, s_35_0);
        // N s_35_2: branch s_35_1 b38 b36
        if s_35_1 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_36_0: const #102552u : u32
        let s_36_0: u32 = 102552;
        // D s_36_1: read-reg s_36_0:struct
        let s_36_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: call _get_HCR_EL2_Type_VI(s_36_1)
        let s_36_2: bool = u_get_HCR_EL2_Type_VI(state, tracer, s_36_1);
        // D s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 1u16);
        // C s_36_4: const #1u : u8
        let s_36_4: bool = true;
        // C s_36_5: cast zx s_36_4 -> bv
        let s_36_5: Bits = Bits::new(s_36_4 as u128, 1u16);
        // D s_36_6: cmp-eq s_36_3 s_36_5
        let s_36_6: bool = ((s_36_3) == (s_36_5));
        // D s_36_7: write-var gs#6690 <= s_36_6
        fn_state.gs_6690 = s_36_6;
        // N s_36_8: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_37_0: read-var gs#6690:u8
        let s_37_0: bool = fn_state.gs_6690;
        // D s_37_1: write-var gs#6691 <= s_37_0
        fn_state.gs_6691 = s_37_0;
        // N s_37_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_38_0: const #1u : u8
        let s_38_0: bool = true;
        // D s_38_1: write-var gs#6690 <= s_38_0
        fn_state.gs_6690 = s_38_0;
        // N s_38_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_39_0: const #102552u : u32
        let s_39_0: u32 = 102552;
        // D s_39_1: read-reg s_39_0:struct
        let s_39_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // D s_39_2: call _get_HCR_EL2_Type_TGE(s_39_1)
        let s_39_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_39_1);
        // D s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // C s_39_4: const #0u : u8
        let s_39_4: bool = false;
        // C s_39_5: cast zx s_39_4 -> bv
        let s_39_5: Bits = Bits::new(s_39_4 as u128, 1u16);
        // D s_39_6: cmp-eq s_39_3 s_39_5
        let s_39_6: bool = ((s_39_3) == (s_39_5));
        // D s_39_7: write-var gs#6688 <= s_39_6
        fn_state.gs_6688 = s_39_6;
        // N s_39_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_40_0: const #16975u : u32
        let s_40_0: u32 = 16975;
        // D s_40_1: read-reg s_40_0:u8
        let s_40_1: u8 = {
            let value = state.read_register::<u8>(s_40_0 as isize);
            tracer.read_register(s_40_0 as isize, value);
            value
        };
        // D s_40_2: cast zx s_40_1 -> bv
        let s_40_2: Bits = Bits::new(s_40_1 as u128, 2u16);
        // C s_40_3: const #448u : u32
        let s_40_3: u32 = 448;
        // D s_40_4: read-reg s_40_3:u8
        let s_40_4: u8 = {
            let value = state.read_register::<u8>(s_40_3 as isize);
            tracer.read_register(s_40_3 as isize, value);
            value
        };
        // D s_40_5: cast zx s_40_4 -> bv
        let s_40_5: Bits = Bits::new(s_40_4 as u128, 2u16);
        // D s_40_6: cmp-eq s_40_2 s_40_5
        let s_40_6: bool = ((s_40_2) == (s_40_5));
        // N s_40_7: branch s_40_6 b43 b41
        if s_40_6 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_41_0: const #16975u : u32
        let s_41_0: u32 = 16975;
        // D s_41_1: read-reg s_41_0:u8
        let s_41_1: u8 = {
            let value = state.read_register::<u8>(s_41_0 as isize);
            tracer.read_register(s_41_0 as isize, value);
            value
        };
        // D s_41_2: cast zx s_41_1 -> bv
        let s_41_2: Bits = Bits::new(s_41_1 as u128, 2u16);
        // C s_41_3: const #440u : u32
        let s_41_3: u32 = 440;
        // D s_41_4: read-reg s_41_3:u8
        let s_41_4: u8 = {
            let value = state.read_register::<u8>(s_41_3 as isize);
            tracer.read_register(s_41_3 as isize, value);
            value
        };
        // D s_41_5: cast zx s_41_4 -> bv
        let s_41_5: Bits = Bits::new(s_41_4 as u128, 2u16);
        // D s_41_6: cmp-eq s_41_2 s_41_5
        let s_41_6: bool = ((s_41_2) == (s_41_5));
        // D s_41_7: write-var gs#6686 <= s_41_6
        fn_state.gs_6686 = s_41_6;
        // N s_41_8: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_42_0: read-var gs#6686:u8
        let s_42_0: bool = fn_state.gs_6686;
        // D s_42_1: write-var gs#6687 <= s_42_0
        fn_state.gs_6687 = s_42_0;
        // N s_42_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_43_0: const #1u : u8
        let s_43_0: bool = true;
        // D s_43_1: write-var gs#6686 <= s_43_0
        fn_state.gs_6686 = s_43_0;
        // N s_43_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // D s_44_1: write-var gs#6685 <= s_44_0
        fn_state.gs_6685 = s_44_0;
        // N s_44_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_45_0: const #1u : u8
        let s_45_0: bool = true;
        // D s_45_1: write-var gs#6684 <= s_45_0
        fn_state.gs_6684 = s_45_0;
        // N s_45_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
