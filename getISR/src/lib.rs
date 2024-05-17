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
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use u_get_HCRX_EL2_Type_VFNMI::*;
use IRQPending::*;
use u_get_HCR_EL2_Type_VSE::*;
use HaveFeatNMI::*;
use Zeros::*;
use u_get_HCRX_EL2_Type_VINMI::*;
use u_get_HCR_EL2_Type_FMO::*;
use IsHCRXEL2Enabled::*;
use IsPhysicalSErrorPending::*;
use Bit::*;
use UsingAArch32::*;
use u_get_HCR_EL2_Type_VI::*;
use u_get_HCR_EL2_Type_VF::*;
use FIQPending::*;
use u_get_HCR_EL2_Type_AMO::*;
use common::*;
pub fn getISR<T: Tracer>(state: &mut State, tracer: &T, gs_140280: ()) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        ga_246660: bool,
        fiq_nmi: bool,
        ga_246662: bool,
        gs_140313: bool,
        value_name: u32,
        gs_140296: bool,
        ga_246651: ProductType8b847afc727d5818,
        ga_246652: ProductType8b847afc727d5818,
        gs_140316: bool,
        gs_140292: bool,
        irq_pending: bool,
        ga_246649: bool,
        ga_246657: bool,
        gs_140293: bool,
        ga_246664: bool,
        gs_140295: bool,
        fiq_pending: bool,
        irq_nmi: bool,
        gs_140289: bool,
        gs_140280: (),
    }
    let fn_state = FunctionState {
        gs_140280,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_0_0: const #32s : i
        let s_0_0: i128 = 32;
        // S s_0_1: call Zeros(s_0_0)
        let s_0_1: Bits = Zeros(state, tracer, s_0_0);
        // S s_0_2: cast reint s_0_1 -> u32
        let s_0_2: u32 = (s_0_1.value() as u32);
        // D s_0_3: write-var value_name <= s_0_2
        fn_state.value_name = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call IsPhysicalSErrorPending(s_0_4)
        let s_0_5: bool = IsPhysicalSErrorPending(state, tracer, s_0_4);
        // N s_0_6: branch s_0_5 b60 b1
        if s_0_5 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var ga#246649 <= s_1_0
        fn_state.ga_246649 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_2_0: read-var ga#246649:u8
        let s_2_0: bool = fn_state.ga_246649;
        // D s_2_1: call Bit(s_2_0)
        let s_2_1: bool = Bit(state, tracer, s_2_0);
        // C s_2_2: const #8s : i
        let s_2_2: i128 = 8;
        // D s_2_3: read-var value_name:u32
        let s_2_3: u32 = fn_state.value_name;
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 32u16);
        // C s_2_5: const #1u : u64
        let s_2_5: u64 = 1;
        // D s_2_6: bit-insert s_2_4 s_2_4 s_2_2 s_2_5
        let s_2_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_2_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_2_4.length(),
            );
            (s_2_4 & mask) | (s_2_4 << s_2_2)
        };
        // D s_2_7: cast reint s_2_6 -> u32
        let s_2_7: u32 = (s_2_6.value() as u32);
        // D s_2_8: write-var value_name <= s_2_7
        fn_state.value_name = s_2_7;
        // C s_2_9: const #() : ()
        let s_2_9: () = ();
        // S s_2_10: call IRQPending(s_2_9)
        let s_2_10: ProductType8b847afc727d5818 = IRQPending(state, tracer, s_2_9);
        // D s_2_11: write-var ga#246651 <= s_2_10
        fn_state.ga_246651 = s_2_10;
        // D s_2_12: read-var ga#246651.0:struct
        let s_2_12: bool = fn_state.ga_246651._0;
        // D s_2_13: read-var ga#246651.1:struct
        let s_2_13: bool = fn_state.ga_246651._1;
        // D s_2_14: write-var irq_pending <= s_2_12
        fn_state.irq_pending = s_2_12;
        // D s_2_15: write-var irq_nmi <= s_2_13
        fn_state.irq_nmi = s_2_13;
        // C s_2_16: const #() : ()
        let s_2_16: () = ();
        // S s_2_17: call FIQPending(s_2_16)
        let s_2_17: ProductType8b847afc727d5818 = FIQPending(state, tracer, s_2_16);
        // D s_2_18: write-var ga#246652 <= s_2_17
        fn_state.ga_246652 = s_2_17;
        // D s_2_19: read-var ga#246652.0:struct
        let s_2_19: bool = fn_state.ga_246652._0;
        // D s_2_20: read-var ga#246652.1:struct
        let s_2_20: bool = fn_state.ga_246652._1;
        // D s_2_21: write-var fiq_pending <= s_2_19
        fn_state.fiq_pending = s_2_19;
        // D s_2_22: write-var fiq_nmi <= s_2_20
        fn_state.fiq_nmi = s_2_20;
        // C s_2_23: const #() : ()
        let s_2_23: () = ();
        // S s_2_24: call HaveFeatNMI(s_2_23)
        let s_2_24: bool = HaveFeatNMI(state, tracer, s_2_23);
        // N s_2_25: branch s_2_24 b59 b3
        if s_2_24 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#140289 <= s_3_0
        fn_state.gs_140289 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_4_0: read-var gs#140289:u8
        let s_4_0: bool = fn_state.gs_140289;
        // N s_4_1: branch s_4_0 b46 b5
        if s_4_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_6_0: read-var irq_pending:u8
        let s_6_0: bool = fn_state.irq_pending;
        // N s_6_1: branch s_6_0 b45 b7
        if s_6_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var ga#246662 <= s_7_0
        fn_state.ga_246662 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_8_0: read-var ga#246662:u8
        let s_8_0: bool = fn_state.ga_246662;
        // D s_8_1: call Bit(s_8_0)
        let s_8_1: bool = Bit(state, tracer, s_8_0);
        // C s_8_2: const #7s : i
        let s_8_2: i128 = 7;
        // D s_8_3: read-var value_name:u32
        let s_8_3: u32 = fn_state.value_name;
        // D s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 32u16);
        // C s_8_5: const #1u : u64
        let s_8_5: u64 = 1;
        // D s_8_6: bit-insert s_8_4 s_8_4 s_8_2 s_8_5
        let s_8_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_8_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_8_4.length(),
            );
            (s_8_4 & mask) | (s_8_4 << s_8_2)
        };
        // D s_8_7: cast reint s_8_6 -> u32
        let s_8_7: u32 = (s_8_6.value() as u32);
        // D s_8_8: write-var value_name <= s_8_7
        fn_state.value_name = s_8_7;
        // D s_8_9: read-var fiq_pending:u8
        let s_8_9: bool = fn_state.fiq_pending;
        // N s_8_10: branch s_8_9 b44 b9
        if s_8_9 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var ga#246664 <= s_9_0
        fn_state.ga_246664 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_10_0: read-var ga#246664:u8
        let s_10_0: bool = fn_state.ga_246664;
        // D s_10_1: call Bit(s_10_0)
        let s_10_1: bool = Bit(state, tracer, s_10_0);
        // C s_10_2: const #6s : i
        let s_10_2: i128 = 6;
        // D s_10_3: read-var value_name:u32
        let s_10_3: u32 = fn_state.value_name;
        // D s_10_4: cast zx s_10_3 -> bv
        let s_10_4: Bits = Bits::new(s_10_3 as u128, 32u16);
        // C s_10_5: const #1u : u64
        let s_10_5: u64 = 1;
        // D s_10_6: bit-insert s_10_4 s_10_4 s_10_2 s_10_5
        let s_10_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_10_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_10_4.length(),
            );
            (s_10_4 & mask) | (s_10_4 << s_10_2)
        };
        // D s_10_7: cast reint s_10_6 -> u32
        let s_10_7: u32 = (s_10_6.value() as u32);
        // D s_10_8: write-var value_name <= s_10_7
        fn_state.value_name = s_10_7;
        // C s_10_9: const #16975u : u32
        let s_10_9: u32 = 16975;
        // D s_10_10: read-reg s_10_9:u8
        let s_10_10: u8 = {
            let value = state.read_register::<u8>(s_10_9 as isize);
            tracer.read_register(s_10_9 as isize, value);
            value
        };
        // D s_10_11: cast zx s_10_10 -> bv
        let s_10_11: Bits = Bits::new(s_10_10 as u128, 2u16);
        // C s_10_12: const #440u : u32
        let s_10_12: u32 = 440;
        // D s_10_13: read-reg s_10_12:u8
        let s_10_13: u8 = {
            let value = state.read_register::<u8>(s_10_12 as isize);
            tracer.read_register(s_10_12 as isize, value);
            value
        };
        // D s_10_14: cast zx s_10_13 -> bv
        let s_10_14: Bits = Bits::new(s_10_13 as u128, 2u16);
        // D s_10_15: cmp-eq s_10_11 s_10_14
        let s_10_15: bool = ((s_10_11) == (s_10_14));
        // N s_10_16: branch s_10_15 b43 b11
        if s_10_15 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#140292 <= s_11_0
        fn_state.gs_140292 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_12_0: read-var gs#140292:u8
        let s_12_0: bool = fn_state.gs_140292;
        // N s_12_1: branch s_12_0 b42 b13
        if s_12_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#140293 <= s_13_0
        fn_state.gs_140293 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_14_0: read-var gs#140293:u8
        let s_14_0: bool = fn_state.gs_140293;
        // N s_14_1: branch s_14_0 b17 b15
        if s_14_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_15_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_16_0: read-var value_name:u32
        let s_16_0: u32 = fn_state.value_name;
        // N s_16_1: return s_16_0
        return s_16_0;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call HaveFeatNMI(s_17_0)
        let s_17_1: bool = HaveFeatNMI(state, tracer, s_17_0);
        // N s_17_2: branch s_17_1 b41 b18
        if s_17_1 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#140295 <= s_18_0
        fn_state.gs_140295 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_19_0: read-var gs#140295:u8
        let s_19_0: bool = fn_state.gs_140295;
        // N s_19_1: branch s_19_0 b40 b20
        if s_19_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#140296 <= s_20_0
        fn_state.gs_140296 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_21_0: read-var gs#140296:u8
        let s_21_0: bool = fn_state.gs_140296;
        // N s_21_1: branch s_21_0 b33 b22
        if s_21_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_22_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_23_0: const #102552u : u32
        let s_23_0: u32 = 102552;
        // D s_23_1: read-reg s_23_0:struct
        let s_23_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call _get_HCR_EL2_Type_AMO(s_23_1)
        let s_23_2: bool = u_get_HCR_EL2_Type_AMO(state, tracer, s_23_1);
        // D s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // C s_23_4: const #1u : u8
        let s_23_4: bool = true;
        // C s_23_5: cast zx s_23_4 -> bv
        let s_23_5: Bits = Bits::new(s_23_4 as u128, 1u16);
        // D s_23_6: cmp-eq s_23_3 s_23_5
        let s_23_6: bool = ((s_23_3) == (s_23_5));
        // N s_23_7: branch s_23_6 b32 b24
        if s_23_6 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_24_0: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_25_0: const #102552u : u32
        let s_25_0: u32 = 102552;
        // D s_25_1: read-reg s_25_0:struct
        let s_25_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: call _get_HCR_EL2_Type_IMO(s_25_1)
        let s_25_2: bool = u_get_HCR_EL2_Type_IMO(state, tracer, s_25_1);
        // D s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // C s_25_4: const #1u : u8
        let s_25_4: bool = true;
        // C s_25_5: cast zx s_25_4 -> bv
        let s_25_5: Bits = Bits::new(s_25_4 as u128, 1u16);
        // D s_25_6: cmp-eq s_25_3 s_25_5
        let s_25_6: bool = ((s_25_3) == (s_25_5));
        // N s_25_7: branch s_25_6 b31 b26
        if s_25_6 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_26_0: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_27_0: const #102552u : u32
        let s_27_0: u32 = 102552;
        // D s_27_1: read-reg s_27_0:struct
        let s_27_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: call _get_HCR_EL2_Type_FMO(s_27_1)
        let s_27_2: bool = u_get_HCR_EL2_Type_FMO(state, tracer, s_27_1);
        // D s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // C s_27_4: const #1u : u8
        let s_27_4: bool = true;
        // C s_27_5: cast zx s_27_4 -> bv
        let s_27_5: Bits = Bits::new(s_27_4 as u128, 1u16);
        // D s_27_6: cmp-eq s_27_3 s_27_5
        let s_27_6: bool = ((s_27_3) == (s_27_5));
        // N s_27_7: branch s_27_6 b30 b28
        if s_27_6 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_28_0: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_29_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_30_0: const #102552u : u32
        let s_30_0: u32 = 102552;
        // D s_30_1: read-reg s_30_0:struct
        let s_30_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // D s_30_2: call _get_HCR_EL2_Type_VF(s_30_1)
        let s_30_2: bool = u_get_HCR_EL2_Type_VF(state, tracer, s_30_1);
        // D s_30_3: call Bit(s_30_2)
        let s_30_3: bool = Bit(state, tracer, s_30_2);
        // C s_30_4: const #6s : i
        let s_30_4: i128 = 6;
        // D s_30_5: read-var value_name:u32
        let s_30_5: u32 = fn_state.value_name;
        // D s_30_6: cast zx s_30_5 -> bv
        let s_30_6: Bits = Bits::new(s_30_5 as u128, 32u16);
        // C s_30_7: const #1u : u64
        let s_30_7: u64 = 1;
        // D s_30_8: bit-insert s_30_6 s_30_6 s_30_4 s_30_7
        let s_30_8: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_30_7 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_30_6.length(),
            );
            (s_30_6 & mask) | (s_30_6 << s_30_4)
        };
        // D s_30_9: cast reint s_30_8 -> u32
        let s_30_9: u32 = (s_30_8.value() as u32);
        // D s_30_10: write-var value_name <= s_30_9
        fn_state.value_name = s_30_9;
        // N s_30_11: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_31_0: const #102552u : u32
        let s_31_0: u32 = 102552;
        // D s_31_1: read-reg s_31_0:struct
        let s_31_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_31_0 as isize);
            tracer.read_register(s_31_0 as isize, value);
            value
        };
        // D s_31_2: call _get_HCR_EL2_Type_VI(s_31_1)
        let s_31_2: bool = u_get_HCR_EL2_Type_VI(state, tracer, s_31_1);
        // D s_31_3: call Bit(s_31_2)
        let s_31_3: bool = Bit(state, tracer, s_31_2);
        // C s_31_4: const #7s : i
        let s_31_4: i128 = 7;
        // D s_31_5: read-var value_name:u32
        let s_31_5: u32 = fn_state.value_name;
        // D s_31_6: cast zx s_31_5 -> bv
        let s_31_6: Bits = Bits::new(s_31_5 as u128, 32u16);
        // C s_31_7: const #1u : u64
        let s_31_7: u64 = 1;
        // D s_31_8: bit-insert s_31_6 s_31_6 s_31_4 s_31_7
        let s_31_8: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_31_7 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_31_6.length(),
            );
            (s_31_6 & mask) | (s_31_6 << s_31_4)
        };
        // D s_31_9: cast reint s_31_8 -> u32
        let s_31_9: u32 = (s_31_8.value() as u32);
        // D s_31_10: write-var value_name <= s_31_9
        fn_state.value_name = s_31_9;
        // N s_31_11: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_32_0: const #8s : i
        let s_32_0: i128 = 8;
        // D s_32_1: read-var value_name:u32
        let s_32_1: u32 = fn_state.value_name;
        // D s_32_2: cast zx s_32_1 -> bv
        let s_32_2: Bits = Bits::new(s_32_1 as u128, 32u16);
        // C s_32_3: const #1u : u64
        let s_32_3: u64 = 1;
        // D s_32_4: bit-extract s_32_2 s_32_0 s_32_3
        let s_32_4: Bits = (Bits::new(
            ((s_32_2) >> (s_32_0)).value(),
            u16::try_from(s_32_3).unwrap(),
        ));
        // D s_32_5: cast reint s_32_4 -> u8
        let s_32_5: bool = ((s_32_4.value()) != 0);
        // C s_32_6: const #0s : i
        let s_32_6: i128 = 0;
        // C s_32_7: const #0u : u64
        let s_32_7: u64 = 0;
        // D s_32_8: cast zx s_32_5 -> u64
        let s_32_8: u64 = (s_32_5 as u64);
        // C s_32_9: const #1u : u64
        let s_32_9: u64 = 1;
        // D s_32_10: and s_32_8 s_32_9
        let s_32_10: u64 = ((s_32_8) & (s_32_9));
        // D s_32_11: cmp-eq s_32_10 s_32_9
        let s_32_11: bool = ((s_32_10) == (s_32_9));
        // D s_32_12: lsl s_32_8 s_32_6
        let s_32_12: u64 = s_32_8 << s_32_6;
        // D s_32_13: or s_32_7 s_32_12
        let s_32_13: u64 = ((s_32_7) | (s_32_12));
        // D s_32_14: cmpl s_32_12
        let s_32_14: u64 = !s_32_12;
        // D s_32_15: and s_32_7 s_32_14
        let s_32_15: u64 = ((s_32_7) & (s_32_14));
        // D s_32_16: select s_32_11 s_32_13 s_32_15
        let s_32_16: u64 = if s_32_11 { s_32_13 } else { s_32_15 };
        // D s_32_17: cast trunc s_32_16 -> u8
        let s_32_17: bool = ((s_32_16) != 0);
        // C s_32_18: const #102552u : u32
        let s_32_18: u32 = 102552;
        // D s_32_19: read-reg s_32_18:struct
        let s_32_19: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_32_18 as isize);
            tracer.read_register(s_32_18 as isize, value);
            value
        };
        // D s_32_20: call _get_HCR_EL2_Type_VSE(s_32_19)
        let s_32_20: bool = u_get_HCR_EL2_Type_VSE(state, tracer, s_32_19);
        // D s_32_21: cast zx s_32_17 -> bv
        let s_32_21: Bits = Bits::new(s_32_17 as u128, 1u16);
        // D s_32_22: cast zx s_32_20 -> bv
        let s_32_22: Bits = Bits::new(s_32_20 as u128, 1u16);
        // D s_32_23: or s_32_21 s_32_22
        let s_32_23: Bits = ((s_32_21) | (s_32_22));
        // D s_32_24: cast reint s_32_23 -> u8
        let s_32_24: bool = ((s_32_23.value()) != 0);
        // D s_32_25: call Bit(s_32_24)
        let s_32_25: bool = Bit(state, tracer, s_32_24);
        // C s_32_26: const #8s : i
        let s_32_26: i128 = 8;
        // D s_32_27: read-var value_name:u32
        let s_32_27: u32 = fn_state.value_name;
        // D s_32_28: cast zx s_32_27 -> bv
        let s_32_28: Bits = Bits::new(s_32_27 as u128, 32u16);
        // C s_32_29: const #1u : u64
        let s_32_29: u64 = 1;
        // D s_32_30: bit-insert s_32_28 s_32_28 s_32_26 s_32_29
        let s_32_30: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_32_29 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_32_28.length(),
            );
            (s_32_28 & mask) | (s_32_28 << s_32_26)
        };
        // D s_32_31: cast reint s_32_30 -> u32
        let s_32_31: u32 = (s_32_30.value() as u32);
        // D s_32_32: write-var value_name <= s_32_31
        fn_state.value_name = s_32_31;
        // N s_32_33: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_33_0: const #102552u : u32
        let s_33_0: u32 = 102552;
        // D s_33_1: read-reg s_33_0:struct
        let s_33_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // D s_33_2: call _get_HCR_EL2_Type_IMO(s_33_1)
        let s_33_2: bool = u_get_HCR_EL2_Type_IMO(state, tracer, s_33_1);
        // C s_33_3: const #102552u : u32
        let s_33_3: u32 = 102552;
        // D s_33_4: read-reg s_33_3:struct
        let s_33_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_33_3 as isize);
            tracer.read_register(s_33_3 as isize, value);
            value
        };
        // D s_33_5: call _get_HCR_EL2_Type_VI(s_33_4)
        let s_33_5: bool = u_get_HCR_EL2_Type_VI(state, tracer, s_33_4);
        // D s_33_6: cast zx s_33_2 -> bv
        let s_33_6: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_7: cast zx s_33_5 -> bv
        let s_33_7: Bits = Bits::new(s_33_5 as u128, 1u16);
        // D s_33_8: and s_33_6 s_33_7
        let s_33_8: Bits = ((s_33_6) & (s_33_7));
        // D s_33_9: cast reint s_33_8 -> u8
        let s_33_9: bool = ((s_33_8.value()) != 0);
        // D s_33_10: cast zx s_33_9 -> bv
        let s_33_10: Bits = Bits::new(s_33_9 as u128, 1u16);
        // C s_33_11: const #1u : u8
        let s_33_11: bool = true;
        // C s_33_12: cast zx s_33_11 -> bv
        let s_33_12: Bits = Bits::new(s_33_11 as u128, 1u16);
        // D s_33_13: cmp-eq s_33_10 s_33_12
        let s_33_13: bool = ((s_33_10) == (s_33_12));
        // N s_33_14: branch s_33_13 b39 b34
        if s_33_13 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_34_0: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_35_0: const #102552u : u32
        let s_35_0: u32 = 102552;
        // D s_35_1: read-reg s_35_0:struct
        let s_35_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_35_0 as isize);
            tracer.read_register(s_35_0 as isize, value);
            value
        };
        // D s_35_2: call _get_HCR_EL2_Type_FMO(s_35_1)
        let s_35_2: bool = u_get_HCR_EL2_Type_FMO(state, tracer, s_35_1);
        // C s_35_3: const #102552u : u32
        let s_35_3: u32 = 102552;
        // D s_35_4: read-reg s_35_3:struct
        let s_35_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_35_3 as isize);
            tracer.read_register(s_35_3 as isize, value);
            value
        };
        // D s_35_5: call _get_HCR_EL2_Type_VF(s_35_4)
        let s_35_5: bool = u_get_HCR_EL2_Type_VF(state, tracer, s_35_4);
        // D s_35_6: cast zx s_35_2 -> bv
        let s_35_6: Bits = Bits::new(s_35_2 as u128, 1u16);
        // D s_35_7: cast zx s_35_5 -> bv
        let s_35_7: Bits = Bits::new(s_35_5 as u128, 1u16);
        // D s_35_8: and s_35_6 s_35_7
        let s_35_8: Bits = ((s_35_6) & (s_35_7));
        // D s_35_9: cast reint s_35_8 -> u8
        let s_35_9: bool = ((s_35_8.value()) != 0);
        // D s_35_10: cast zx s_35_9 -> bv
        let s_35_10: Bits = Bits::new(s_35_9 as u128, 1u16);
        // C s_35_11: const #1u : u8
        let s_35_11: bool = true;
        // C s_35_12: cast zx s_35_11 -> bv
        let s_35_12: Bits = Bits::new(s_35_11 as u128, 1u16);
        // D s_35_13: cmp-eq s_35_10 s_35_12
        let s_35_13: bool = ((s_35_10) == (s_35_12));
        // N s_35_14: branch s_35_13 b38 b36
        if s_35_13 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_36_0: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_37_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_38_0: const #22528u : u32
        let s_38_0: u32 = 22528;
        // D s_38_1: read-reg s_38_0:struct
        let s_38_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // D s_38_2: call _get_HCRX_EL2_Type_VFNMI(s_38_1)
        let s_38_2: bool = u_get_HCRX_EL2_Type_VFNMI(state, tracer, s_38_1);
        // D s_38_3: call Bit(s_38_2)
        let s_38_3: bool = Bit(state, tracer, s_38_2);
        // C s_38_4: const #9s : i
        let s_38_4: i128 = 9;
        // D s_38_5: read-var value_name:u32
        let s_38_5: u32 = fn_state.value_name;
        // D s_38_6: cast zx s_38_5 -> bv
        let s_38_6: Bits = Bits::new(s_38_5 as u128, 32u16);
        // C s_38_7: const #1u : u64
        let s_38_7: u64 = 1;
        // D s_38_8: bit-insert s_38_6 s_38_6 s_38_4 s_38_7
        let s_38_8: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_38_7 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_38_6.length(),
            );
            (s_38_6 & mask) | (s_38_6 << s_38_4)
        };
        // D s_38_9: cast reint s_38_8 -> u32
        let s_38_9: u32 = (s_38_8.value() as u32);
        // D s_38_10: write-var value_name <= s_38_9
        fn_state.value_name = s_38_9;
        // N s_38_11: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_39_0: const #22528u : u32
        let s_39_0: u32 = 22528;
        // D s_39_1: read-reg s_39_0:struct
        let s_39_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // D s_39_2: call _get_HCRX_EL2_Type_VINMI(s_39_1)
        let s_39_2: bool = u_get_HCRX_EL2_Type_VINMI(state, tracer, s_39_1);
        // D s_39_3: call Bit(s_39_2)
        let s_39_3: bool = Bit(state, tracer, s_39_2);
        // C s_39_4: const #10s : i
        let s_39_4: i128 = 10;
        // D s_39_5: read-var value_name:u32
        let s_39_5: u32 = fn_state.value_name;
        // D s_39_6: cast zx s_39_5 -> bv
        let s_39_6: Bits = Bits::new(s_39_5 as u128, 32u16);
        // C s_39_7: const #1u : u64
        let s_39_7: u64 = 1;
        // D s_39_8: bit-insert s_39_6 s_39_6 s_39_4 s_39_7
        let s_39_8: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_39_7 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_39_6.length(),
            );
            (s_39_6 & mask) | (s_39_6 << s_39_4)
        };
        // D s_39_9: cast reint s_39_8 -> u32
        let s_39_9: u32 = (s_39_8.value() as u32);
        // D s_39_10: write-var value_name <= s_39_9
        fn_state.value_name = s_39_9;
        // N s_39_11: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call IsHCRXEL2Enabled(s_40_0)
        let s_40_1: bool = IsHCRXEL2Enabled(state, tracer, s_40_0);
        // D s_40_2: write-var gs#140296 <= s_40_1
        fn_state.gs_140296 = s_40_1;
        // N s_40_3: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_41_0: const #() : ()
        let s_41_0: () = ();
        // S s_41_1: call UsingAArch32(s_41_0)
        let s_41_1: bool = UsingAArch32(state, tracer, s_41_0);
        // S s_41_2: not s_41_1
        let s_41_2: bool = !s_41_1;
        // D s_41_3: write-var gs#140295 <= s_41_2
        fn_state.gs_140295 = s_41_2;
        // N s_41_4: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_42_0: const #102552u : u32
        let s_42_0: u32 = 102552;
        // D s_42_1: read-reg s_42_0:struct
        let s_42_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // D s_42_2: call _get_HCR_EL2_Type_TGE(s_42_1)
        let s_42_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_42_1);
        // D s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 1u16);
        // C s_42_4: const #0u : u8
        let s_42_4: bool = false;
        // C s_42_5: cast zx s_42_4 -> bv
        let s_42_5: Bits = Bits::new(s_42_4 as u128, 1u16);
        // D s_42_6: cmp-eq s_42_3 s_42_5
        let s_42_6: bool = ((s_42_3) == (s_42_5));
        // D s_42_7: write-var gs#140293 <= s_42_6
        fn_state.gs_140293 = s_42_6;
        // N s_42_8: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call EL2Enabled(s_43_0)
        let s_43_1: bool = EL2Enabled(state, tracer, s_43_0);
        // D s_43_2: write-var gs#140292 <= s_43_1
        fn_state.gs_140292 = s_43_1;
        // N s_43_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // D s_44_1: write-var ga#246664 <= s_44_0
        fn_state.ga_246664 = s_44_0;
        // N s_44_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_45_0: const #1u : u8
        let s_45_0: bool = true;
        // D s_45_1: write-var ga#246662 <= s_45_0
        fn_state.ga_246662 = s_45_0;
        // N s_45_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_46_0: read-var irq_pending:u8
        let s_46_0: bool = fn_state.irq_pending;
        // N s_46_1: branch s_46_0 b58 b47
        if s_46_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_47_0: const #0u : u8
        let s_47_0: bool = false;
        // D s_47_1: write-var gs#140313 <= s_47_0
        fn_state.gs_140313 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_48_0: read-var gs#140313:u8
        let s_48_0: bool = fn_state.gs_140313;
        // N s_48_1: branch s_48_0 b57 b49
        if s_48_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var ga#246657 <= s_49_0
        fn_state.ga_246657 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_50_0: read-var ga#246657:u8
        let s_50_0: bool = fn_state.ga_246657;
        // D s_50_1: call Bit(s_50_0)
        let s_50_1: bool = Bit(state, tracer, s_50_0);
        // C s_50_2: const #10s : i
        let s_50_2: i128 = 10;
        // D s_50_3: read-var value_name:u32
        let s_50_3: u32 = fn_state.value_name;
        // D s_50_4: cast zx s_50_3 -> bv
        let s_50_4: Bits = Bits::new(s_50_3 as u128, 32u16);
        // C s_50_5: const #1u : u64
        let s_50_5: u64 = 1;
        // D s_50_6: bit-insert s_50_4 s_50_4 s_50_2 s_50_5
        let s_50_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_50_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_50_4.length(),
            );
            (s_50_4 & mask) | (s_50_4 << s_50_2)
        };
        // D s_50_7: cast reint s_50_6 -> u32
        let s_50_7: u32 = (s_50_6.value() as u32);
        // D s_50_8: write-var value_name <= s_50_7
        fn_state.value_name = s_50_7;
        // D s_50_9: read-var fiq_pending:u8
        let s_50_9: bool = fn_state.fiq_pending;
        // N s_50_10: branch s_50_9 b56 b51
        if s_50_9 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_51_0: const #0u : u8
        let s_51_0: bool = false;
        // D s_51_1: write-var gs#140316 <= s_51_0
        fn_state.gs_140316 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_52_0: read-var gs#140316:u8
        let s_52_0: bool = fn_state.gs_140316;
        // N s_52_1: branch s_52_0 b55 b53
        if s_52_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var ga#246660 <= s_53_0
        fn_state.ga_246660 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_54_0: read-var ga#246660:u8
        let s_54_0: bool = fn_state.ga_246660;
        // D s_54_1: call Bit(s_54_0)
        let s_54_1: bool = Bit(state, tracer, s_54_0);
        // C s_54_2: const #9s : i
        let s_54_2: i128 = 9;
        // D s_54_3: read-var value_name:u32
        let s_54_3: u32 = fn_state.value_name;
        // D s_54_4: cast zx s_54_3 -> bv
        let s_54_4: Bits = Bits::new(s_54_3 as u128, 32u16);
        // C s_54_5: const #1u : u64
        let s_54_5: u64 = 1;
        // D s_54_6: bit-insert s_54_4 s_54_4 s_54_2 s_54_5
        let s_54_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_54_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_54_4.length(),
            );
            (s_54_4 & mask) | (s_54_4 << s_54_2)
        };
        // D s_54_7: cast reint s_54_6 -> u32
        let s_54_7: u32 = (s_54_6.value() as u32);
        // D s_54_8: write-var value_name <= s_54_7
        fn_state.value_name = s_54_7;
        // N s_54_9: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_55_0: const #1u : u8
        let s_55_0: bool = true;
        // D s_55_1: write-var ga#246660 <= s_55_0
        fn_state.ga_246660 = s_55_0;
        // N s_55_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_56_0: read-var fiq_nmi:u8
        let s_56_0: bool = fn_state.fiq_nmi;
        // D s_56_1: write-var gs#140316 <= s_56_0
        fn_state.gs_140316 = s_56_0;
        // N s_56_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_57_0: const #1u : u8
        let s_57_0: bool = true;
        // D s_57_1: write-var ga#246657 <= s_57_0
        fn_state.ga_246657 = s_57_0;
        // N s_57_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_58_0: read-var irq_nmi:u8
        let s_58_0: bool = fn_state.irq_nmi;
        // D s_58_1: write-var gs#140313 <= s_58_0
        fn_state.gs_140313 = s_58_0;
        // N s_58_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_59_0: const #() : ()
        let s_59_0: () = ();
        // S s_59_1: call UsingAArch32(s_59_0)
        let s_59_1: bool = UsingAArch32(state, tracer, s_59_0);
        // S s_59_2: not s_59_1
        let s_59_2: bool = !s_59_1;
        // D s_59_3: write-var gs#140289 <= s_59_2
        fn_state.gs_140289 = s_59_2;
        // N s_59_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_60_0: const #1u : u8
        let s_60_0: bool = true;
        // D s_60_1: write-var ga#246649 <= s_60_0
        fn_state.ga_246649 = s_60_0;
        // N s_60_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
