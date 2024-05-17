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
use DFAR_write::*;
use TTBCR_read::*;
use IsExternalSyncAbort__1::*;
use Mk_IFSR_Type::*;
use Mk_DFSR_Type::*;
use DFSR_write::*;
use u__IMPDEF_boolean::*;
use CurrentSecurityState::*;
use AArch32_CommonFaultStatus::*;
use u__UNKNOWN_bits::*;
use EncodeSDFSC::*;
use IFSR_write::*;
use Bit::*;
use u_get_TTBCR_Type_EAE::*;
use DFAR_S_write::*;
use common::*;
pub fn AArch32_ReportDataAbort<T: Tracer>(
    state: &mut State,
    tracer: &T,
    route_to_monitor: bool,
    fault: ProductType1d757adad216cdef,
    vaddress: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_6484: bool,
        gs_9284: bool,
        gs_9286: bool,
        gs_9285: bool,
        ga_6474: ProductType9878976b5bcce9c9,
        i_syndrome: u32,
        gs_9288: bool,
        ga_6488: ProductType9878976b5bcce9c9,
        long_format: bool,
        gs_9295: bool,
        gs_9290: bool,
        gs_9287: bool,
        gs_9282: bool,
        ga_6476: ProductType9878976b5bcce9c9,
        gs_9289: bool,
        syndrome: u32,
        ga_6472: ProductType9878976b5bcce9c9,
        route_to_monitor: bool,
        fault: ProductType1d757adad216cdef,
        vaddress: u32,
    }
    let fn_state = FunctionState {
        route_to_monitor,
        fault,
        vaddress,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var route_to_monitor:u8
        let s_0_0: bool = fn_state.route_to_monitor;
        // N s_0_1: branch s_0_0 b49 b1
        if s_0_0 {
            return block_49(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#9282 <= s_1_0
        fn_state.gs_9282 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#9282:u8
        let s_2_0: bool = fn_state.gs_9282;
        // N s_2_1: branch s_2_0 b33 b3
        if s_2_0 {
            return block_33(state, tracer, fn_state);
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
        // S s_3_1: call TTBCR_read(s_3_0)
        let s_3_1: ProductType700c18a878c5601b = TTBCR_read(state, tracer, s_3_0);
        // S s_3_2: call _get_TTBCR_Type_EAE(s_3_1)
        let s_3_2: bool = u_get_TTBCR_Type_EAE(state, tracer, s_3_1);
        // S s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // C s_3_4: const #1u : u8
        let s_3_4: bool = true;
        // C s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 1u16);
        // S s_3_6: cmp-eq s_3_3 s_3_5
        let s_3_6: bool = ((s_3_3) == (s_3_5));
        // D s_3_7: write-var long_format <= s_3_6
        fn_state.long_format = s_3_6;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var fault:struct
        let s_4_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_4_1: read-var long_format:u8
        let s_4_1: bool = fn_state.long_format;
        // D s_4_2: call AArch32_CommonFaultStatus(s_4_0, s_4_1)
        let s_4_2: u32 = AArch32_CommonFaultStatus(state, tracer, s_4_0, s_4_1);
        // D s_4_3: write-var syndrome <= s_4_2
        fn_state.syndrome = s_4_2;
        // D s_4_4: read-var fault.0:struct
        let s_4_4: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_4_5: write-var ga#6472 <= s_4_4
        fn_state.ga_6472 = s_4_4;
        // D s_4_6: read-var ga#6472.1:struct
        let s_4_6: u32 = fn_state.ga_6472._1;
        // C s_4_7: const #6u : u32
        let s_4_7: u32 = 6;
        // D s_4_8: cmp-eq s_4_6 s_4_7
        let s_4_8: bool = ((s_4_6) == (s_4_7));
        // N s_4_9: branch s_4_8 b32 b5
        if s_4_8 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var fault.0:struct
        let s_5_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_5_1: write-var ga#6474 <= s_5_0
        fn_state.ga_6474 = s_5_0;
        // D s_5_2: read-var ga#6474.1:struct
        let s_5_2: u32 = fn_state.ga_6474._1;
        // C s_5_3: const #5u : u32
        let s_5_3: u32 = 5;
        // D s_5_4: cmp-eq s_5_2 s_5_3
        let s_5_4: bool = ((s_5_2) == (s_5_3));
        // N s_5_5: branch s_5_4 b31 b6
        if s_5_4 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var fault.0:struct
        let s_6_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_6_1: write-var ga#6476 <= s_6_0
        fn_state.ga_6476 = s_6_0;
        // D s_6_2: read-var ga#6476.1:struct
        let s_6_2: u32 = fn_state.ga_6476._1;
        // C s_6_3: const #8u : u32
        let s_6_3: u32 = 8;
        // D s_6_4: cmp-eq s_6_2 s_6_3
        let s_6_4: bool = ((s_6_2) == (s_6_3));
        // D s_6_5: write-var gs#9289 <= s_6_4
        fn_state.gs_9289 = s_6_4;
        // N s_6_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#9289:u8
        let s_7_0: bool = fn_state.gs_9289;
        // D s_7_1: write-var gs#9290 <= s_7_0
        fn_state.gs_9290 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#9290:u8
        let s_8_0: bool = fn_state.gs_9290;
        // N s_8_1: branch s_8_0 b30 b9
        if s_8_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var fault.19:struct
        let s_9_0: bool = fn_state.fault._19;
        // N s_9_1: branch s_9_0 b29 b10
        if s_9_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_10_1: write-var ga#6484 <= s_10_0
        fn_state.ga_6484 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var ga#6484:u8
        let s_11_0: bool = fn_state.ga_6484;
        // D s_11_1: call Bit(s_11_0)
        let s_11_1: bool = Bit(state, tracer, s_11_0);
        // C s_11_2: const #11s : i
        let s_11_2: i128 = 11;
        // D s_11_3: read-var syndrome:u32
        let s_11_3: u32 = fn_state.syndrome;
        // D s_11_4: cast zx s_11_3 -> bv
        let s_11_4: Bits = Bits::new(s_11_3 as u128, 32u16);
        // C s_11_5: const #1u : u64
        let s_11_5: u64 = 1;
        // D s_11_6: bit-insert s_11_4 s_11_4 s_11_2 s_11_5
        let s_11_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_11_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_11_4.length(),
            );
            (s_11_4 & mask) | (s_11_4 << s_11_2)
        };
        // D s_11_7: cast reint s_11_6 -> u32
        let s_11_7: u32 = (s_11_6.value() as u32);
        // D s_11_8: write-var syndrome <= s_11_7
        fn_state.syndrome = s_11_7;
        // N s_11_9: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var long_format:u8
        let s_12_0: bool = fn_state.long_format;
        // D s_12_1: not s_12_0
        let s_12_1: bool = !s_12_0;
        // N s_12_2: branch s_12_1 b28 b13
        if s_12_1 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var fault.0:struct
        let s_14_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_14_1: write-var ga#6488 <= s_14_0
        fn_state.ga_6488 = s_14_0;
        // D s_14_2: read-var ga#6488.1:struct
        let s_14_2: u32 = fn_state.ga_6488._1;
        // C s_14_3: const #5u : u32
        let s_14_3: u32 = 5;
        // D s_14_4: cmp-eq s_14_2 s_14_3
        let s_14_4: bool = ((s_14_2) == (s_14_3));
        // N s_14_5: branch s_14_4 b19 b15
        if s_14_4 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var route_to_monitor:u8
        let s_16_0: bool = fn_state.route_to_monitor;
        // N s_16_1: branch s_16_0 b18 b17
        if s_16_0 {
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
        // D s_17_0: read-var syndrome:u32
        let s_17_0: u32 = fn_state.syndrome;
        // D s_17_1: call Mk_DFSR_Type(s_17_0)
        let s_17_1: ProductType700c18a878c5601b = Mk_DFSR_Type(state, tracer, s_17_0);
        // D s_17_2: call DFSR_write(s_17_1)
        let s_17_2: () = DFSR_write(state, tracer, s_17_1);
        // D s_17_3: read-var vaddress:u32
        let s_17_3: u32 = fn_state.vaddress;
        // D s_17_4: call DFAR_write(s_17_3)
        let s_17_4: () = DFAR_write(state, tracer, s_17_3);
        // N s_17_5: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var syndrome:u32
        let s_18_0: u32 = fn_state.syndrome;
        // D s_18_1: call Mk_DFSR_Type(s_18_0)
        let s_18_1: ProductType700c18a878c5601b = Mk_DFSR_Type(state, tracer, s_18_0);
        // C s_18_2: const #103400u : u32
        let s_18_2: u32 = 103400;
        // N s_18_3: write-reg s_18_2 <= s_18_1
        let s_18_3: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_18_2 as isize, s_18_1);
            tracer.write_register(s_18_2 as isize, s_18_1);
        };
        // D s_18_4: read-var vaddress:u32
        let s_18_4: u32 = fn_state.vaddress;
        // D s_18_5: call DFAR_S_write(s_18_4)
        let s_18_5: () = DFAR_S_write(state, tracer, s_18_4);
        // N s_18_6: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var long_format:u8
        let s_19_0: bool = fn_state.long_format;
        // D s_19_1: not s_19_0
        let s_19_1: bool = !s_19_0;
        // N s_19_2: branch s_19_1 b27 b20
        if s_19_1 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#9295 <= s_20_0
        fn_state.gs_9295 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#9295:u8
        let s_21_0: bool = fn_state.gs_9295;
        // N s_21_1: branch s_21_0 b26 b22
        if s_21_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #32s : i64
        let s_22_0: i64 = 32;
        // C s_22_1: cast zx s_22_0 -> i
        let s_22_1: i128 = (i128::try_from(s_22_0).unwrap());
        // S s_22_2: call __UNKNOWN_bits(s_22_1)
        let s_22_2: Bits = u__UNKNOWN_bits(state, tracer, s_22_1);
        // S s_22_3: cast reint s_22_2 -> u32
        let s_22_3: u32 = (s_22_2.value() as u32);
        // D s_22_4: write-var i_syndrome <= s_22_3
        fn_state.i_syndrome = s_22_3;
        // N s_22_5: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var route_to_monitor:u8
        let s_23_0: bool = fn_state.route_to_monitor;
        // N s_23_1: branch s_23_0 b25 b24
        if s_23_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var i_syndrome:u32
        let s_24_0: u32 = fn_state.i_syndrome;
        // D s_24_1: call Mk_IFSR_Type(s_24_0)
        let s_24_1: ProductType700c18a878c5601b = Mk_IFSR_Type(state, tracer, s_24_0);
        // D s_24_2: call IFSR_write(s_24_1)
        let s_24_2: () = IFSR_write(state, tracer, s_24_1);
        // N s_24_3: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var i_syndrome:u32
        let s_25_0: u32 = fn_state.i_syndrome;
        // D s_25_1: call Mk_IFSR_Type(s_25_0)
        let s_25_1: ProductType700c18a878c5601b = Mk_IFSR_Type(state, tracer, s_25_0);
        // C s_25_2: const #11016u : u32
        let s_25_2: u32 = 11016;
        // N s_25_3: write-reg s_25_2 <= s_25_1
        let s_25_3: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_25_2 as isize, s_25_1);
            tracer.write_register(s_25_2 as isize, s_25_1);
        };
        // N s_25_4: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var syndrome:u32
        let s_26_0: u32 = fn_state.syndrome;
        // D s_26_1: write-var i_syndrome <= s_26_0
        fn_state.i_syndrome = s_26_0;
        // C s_26_2: const #1s : i
        let s_26_2: i128 = 1;
        // C s_26_3: const #23u : u32
        let s_26_3: u32 = 23;
        // S s_26_4: call EncodeSDFSC(s_26_3, s_26_2)
        let s_26_4: u8 = EncodeSDFSC(state, tracer, s_26_3, s_26_2);
        // C s_26_5: const #4s : i
        let s_26_5: i128 = 4;
        // S s_26_6: cast zx s_26_4 -> bv
        let s_26_6: Bits = Bits::new(s_26_4 as u128, 5u16);
        // C s_26_7: const #1s : i64
        let s_26_7: i64 = 1;
        // C s_26_8: cast zx s_26_7 -> i
        let s_26_8: i128 = (i128::try_from(s_26_7).unwrap());
        // C s_26_9: const #0s : i
        let s_26_9: i128 = 0;
        // C s_26_10: add s_26_9 s_26_8
        let s_26_10: i128 = (s_26_9 + s_26_8);
        // D s_26_11: bit-extract s_26_6 s_26_5 s_26_10
        let s_26_11: Bits = (Bits::new(
            ((s_26_6) >> (s_26_5)).value(),
            u16::try_from(s_26_10).unwrap(),
        ));
        // D s_26_12: cast reint s_26_11 -> u8
        let s_26_12: bool = ((s_26_11.value()) != 0);
        // C s_26_13: const #10s : i
        let s_26_13: i128 = 10;
        // D s_26_14: read-var syndrome:u32
        let s_26_14: u32 = fn_state.syndrome;
        // D s_26_15: cast zx s_26_14 -> bv
        let s_26_15: Bits = Bits::new(s_26_14 as u128, 32u16);
        // D s_26_16: cast zx s_26_12 -> bv
        let s_26_16: Bits = Bits::new(s_26_12 as u128, 1u16);
        // C s_26_17: const #0s : i
        let s_26_17: i128 = 0;
        // C s_26_18: const #1u : u64
        let s_26_18: u64 = 1;
        // C s_26_19: cast zx s_26_18 -> bv
        let s_26_19: Bits = Bits::new(s_26_18 as u128, 64u16);
        // C s_26_20: lsl s_26_19 s_26_17
        let s_26_20: Bits = s_26_19 << s_26_17;
        // C s_26_21: sub s_26_20 s_26_19
        let s_26_21: Bits = ((s_26_20) - (s_26_19));
        // D s_26_22: and s_26_16 s_26_21
        let s_26_22: Bits = ((s_26_16) & (s_26_21));
        // D s_26_23: lsl s_26_22 s_26_13
        let s_26_23: Bits = s_26_22 << s_26_13;
        // C s_26_24: lsl s_26_21 s_26_13
        let s_26_24: Bits = s_26_21 << s_26_13;
        // C s_26_25: cmpl s_26_24
        let s_26_25: Bits = !s_26_24;
        // D s_26_26: and s_26_15 s_26_25
        let s_26_26: Bits = ((s_26_15) & (s_26_25));
        // D s_26_27: or s_26_26 s_26_23
        let s_26_27: Bits = ((s_26_26) | (s_26_23));
        // D s_26_28: cast reint s_26_27 -> u32
        let s_26_28: u32 = (s_26_27.value() as u32);
        // D s_26_29: write-var syndrome <= s_26_28
        fn_state.syndrome = s_26_28;
        // C s_26_30: const #0s : i
        let s_26_30: i128 = 0;
        // S s_26_31: cast zx s_26_4 -> bv
        let s_26_31: Bits = Bits::new(s_26_4 as u128, 5u16);
        // C s_26_32: const #1s : i64
        let s_26_32: i64 = 1;
        // C s_26_33: cast zx s_26_32 -> i
        let s_26_33: i128 = (i128::try_from(s_26_32).unwrap());
        // C s_26_34: const #3s : i
        let s_26_34: i128 = 3;
        // C s_26_35: add s_26_34 s_26_33
        let s_26_35: i128 = (s_26_34 + s_26_33);
        // D s_26_36: bit-extract s_26_31 s_26_30 s_26_35
        let s_26_36: Bits = (Bits::new(
            ((s_26_31) >> (s_26_30)).value(),
            u16::try_from(s_26_35).unwrap(),
        ));
        // D s_26_37: cast reint s_26_36 -> u8
        let s_26_37: u8 = (s_26_36.value() as u8);
        // C s_26_38: const #0s : i
        let s_26_38: i128 = 0;
        // D s_26_39: read-var syndrome:u32
        let s_26_39: u32 = fn_state.syndrome;
        // D s_26_40: cast zx s_26_39 -> bv
        let s_26_40: Bits = Bits::new(s_26_39 as u128, 32u16);
        // D s_26_41: cast zx s_26_37 -> bv
        let s_26_41: Bits = Bits::new(s_26_37 as u128, 4u16);
        // C s_26_42: const #3s : i
        let s_26_42: i128 = 3;
        // C s_26_43: const #1u : u64
        let s_26_43: u64 = 1;
        // C s_26_44: cast zx s_26_43 -> bv
        let s_26_44: Bits = Bits::new(s_26_43 as u128, 64u16);
        // C s_26_45: lsl s_26_44 s_26_42
        let s_26_45: Bits = s_26_44 << s_26_42;
        // C s_26_46: sub s_26_45 s_26_44
        let s_26_46: Bits = ((s_26_45) - (s_26_44));
        // D s_26_47: and s_26_41 s_26_46
        let s_26_47: Bits = ((s_26_41) & (s_26_46));
        // D s_26_48: lsl s_26_47 s_26_38
        let s_26_48: Bits = s_26_47 << s_26_38;
        // C s_26_49: lsl s_26_46 s_26_38
        let s_26_49: Bits = s_26_46 << s_26_38;
        // C s_26_50: cmpl s_26_49
        let s_26_50: Bits = !s_26_49;
        // D s_26_51: and s_26_40 s_26_50
        let s_26_51: Bits = ((s_26_40) & (s_26_50));
        // D s_26_52: or s_26_51 s_26_48
        let s_26_52: Bits = ((s_26_51) | (s_26_48));
        // D s_26_53: cast reint s_26_52 -> u32
        let s_26_53: u32 = (s_26_52.value() as u32);
        // D s_26_54: write-var syndrome <= s_26_53
        fn_state.syndrome = s_26_53;
        // N s_26_55: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #"Report I-cache maintenance fault in IFSR" : str
        let s_27_0: &'static str = "Report I-cache maintenance fault in IFSR";
        // S s_27_1: call __IMPDEF_boolean(s_27_0)
        let s_27_1: bool = u__IMPDEF_boolean(state, tracer, s_27_0);
        // D s_27_2: write-var gs#9295 <= s_27_1
        fn_state.gs_9295 = s_27_1;
        // N s_27_3: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var fault.4:struct
        let s_28_0: u8 = fn_state.fault._4;
        // C s_28_1: const #4s : i
        let s_28_1: i128 = 4;
        // D s_28_2: read-var syndrome:u32
        let s_28_2: u32 = fn_state.syndrome;
        // D s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 32u16);
        // D s_28_4: cast zx s_28_0 -> bv
        let s_28_4: Bits = Bits::new(s_28_0 as u128, 4u16);
        // C s_28_5: const #3s : i
        let s_28_5: i128 = 3;
        // C s_28_6: const #1u : u64
        let s_28_6: u64 = 1;
        // C s_28_7: cast zx s_28_6 -> bv
        let s_28_7: Bits = Bits::new(s_28_6 as u128, 64u16);
        // C s_28_8: lsl s_28_7 s_28_5
        let s_28_8: Bits = s_28_7 << s_28_5;
        // C s_28_9: sub s_28_8 s_28_7
        let s_28_9: Bits = ((s_28_8) - (s_28_7));
        // D s_28_10: and s_28_4 s_28_9
        let s_28_10: Bits = ((s_28_4) & (s_28_9));
        // D s_28_11: lsl s_28_10 s_28_1
        let s_28_11: Bits = s_28_10 << s_28_1;
        // C s_28_12: lsl s_28_9 s_28_1
        let s_28_12: Bits = s_28_9 << s_28_1;
        // C s_28_13: cmpl s_28_12
        let s_28_13: Bits = !s_28_12;
        // D s_28_14: and s_28_3 s_28_13
        let s_28_14: Bits = ((s_28_3) & (s_28_13));
        // D s_28_15: or s_28_14 s_28_11
        let s_28_15: Bits = ((s_28_14) | (s_28_11));
        // D s_28_16: cast reint s_28_15 -> u32
        let s_28_16: u32 = (s_28_15.value() as u32);
        // D s_28_17: write-var syndrome <= s_28_16
        fn_state.syndrome = s_28_16;
        // N s_28_18: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var ga#6484 <= s_29_0
        fn_state.ga_6484 = s_29_0;
        // N s_29_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // S s_30_1: call Bit(s_30_0)
        let s_30_1: bool = Bit(state, tracer, s_30_0);
        // C s_30_2: const #13s : i
        let s_30_2: i128 = 13;
        // D s_30_3: read-var syndrome:u32
        let s_30_3: u32 = fn_state.syndrome;
        // D s_30_4: cast zx s_30_3 -> bv
        let s_30_4: Bits = Bits::new(s_30_3 as u128, 32u16);
        // C s_30_5: const #1u : u64
        let s_30_5: u64 = 1;
        // D s_30_6: bit-insert s_30_4 s_30_4 s_30_2 s_30_5
        let s_30_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_30_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_30_4.length(),
            );
            (s_30_4 & mask) | (s_30_4 << s_30_2)
        };
        // D s_30_7: cast reint s_30_6 -> u32
        let s_30_7: u32 = (s_30_6.value() as u32);
        // D s_30_8: write-var syndrome <= s_30_7
        fn_state.syndrome = s_30_7;
        // C s_30_9: const #1u : u8
        let s_30_9: bool = true;
        // S s_30_10: call Bit(s_30_9)
        let s_30_10: bool = Bit(state, tracer, s_30_9);
        // C s_30_11: const #11s : i
        let s_30_11: i128 = 11;
        // D s_30_12: read-var syndrome:u32
        let s_30_12: u32 = fn_state.syndrome;
        // D s_30_13: cast zx s_30_12 -> bv
        let s_30_13: Bits = Bits::new(s_30_12 as u128, 32u16);
        // C s_30_14: const #1u : u64
        let s_30_14: u64 = 1;
        // D s_30_15: bit-insert s_30_13 s_30_13 s_30_11 s_30_14
        let s_30_15: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_30_14 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_30_13.length(),
            );
            (s_30_13 & mask) | (s_30_13 << s_30_11)
        };
        // D s_30_16: cast reint s_30_15 -> u32
        let s_30_16: u32 = (s_30_15.value() as u32);
        // D s_30_17: write-var syndrome <= s_30_16
        fn_state.syndrome = s_30_16;
        // N s_30_18: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: write-var gs#9289 <= s_31_0
        fn_state.gs_9289 = s_31_0;
        // N s_31_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #1u : u8
        let s_32_0: bool = true;
        // D s_32_1: write-var gs#9290 <= s_32_0
        fn_state.gs_9290 = s_32_0;
        // N s_32_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #15368u : u32
        let s_33_0: u32 = 15368;
        // D s_33_1: read-reg s_33_0:struct
        let s_33_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // D s_33_2: call _get_TTBCR_Type_EAE(s_33_1)
        let s_33_2: bool = u_get_TTBCR_Type_EAE(state, tracer, s_33_1);
        // D s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // C s_33_4: const #1u : u8
        let s_33_4: bool = true;
        // C s_33_5: cast zx s_33_4 -> bv
        let s_33_5: Bits = Bits::new(s_33_4 as u128, 1u16);
        // D s_33_6: cmp-eq s_33_3 s_33_5
        let s_33_6: bool = ((s_33_3) == (s_33_5));
        // N s_33_7: branch s_33_6 b48 b34
        if s_33_6 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var fault:struct
        let s_34_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_34_1: call IsExternalSyncAbort__1(s_34_0)
        let s_34_1: bool = IsExternalSyncAbort__1(state, tracer, s_34_0);
        // N s_34_2: branch s_34_1 b38 b35
        if s_34_1 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#9287 <= s_35_0
        fn_state.gs_9287 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#9287:u8
        let s_36_0: bool = fn_state.gs_9287;
        // D s_36_1: write-var gs#9288 <= s_36_0
        fn_state.gs_9288 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#9288:u8
        let s_37_0: bool = fn_state.gs_9288;
        // D s_37_1: write-var long_format <= s_37_0
        fn_state.long_format = s_37_0;
        // N s_37_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #16975u : u32
        let s_38_0: u32 = 16975;
        // D s_38_1: read-reg s_38_0:u8
        let s_38_1: u8 = {
            let value = state.read_register::<u8>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // D s_38_2: cast zx s_38_1 -> bv
        let s_38_2: Bits = Bits::new(s_38_1 as u128, 2u16);
        // C s_38_3: const #432u : u32
        let s_38_3: u32 = 432;
        // D s_38_4: read-reg s_38_3:u8
        let s_38_4: u8 = {
            let value = state.read_register::<u8>(s_38_3 as isize);
            tracer.read_register(s_38_3 as isize, value);
            value
        };
        // D s_38_5: cast zx s_38_4 -> bv
        let s_38_5: Bits = Bits::new(s_38_4 as u128, 2u16);
        // D s_38_6: cmp-eq s_38_2 s_38_5
        let s_38_6: bool = ((s_38_2) == (s_38_5));
        // N s_38_7: branch s_38_6 b47 b39
        if s_38_6 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #() : ()
        let s_39_0: () = ();
        // S s_39_1: call TTBCR_read(s_39_0)
        let s_39_1: ProductType700c18a878c5601b = TTBCR_read(state, tracer, s_39_0);
        // S s_39_2: call _get_TTBCR_Type_EAE(s_39_1)
        let s_39_2: bool = u_get_TTBCR_Type_EAE(state, tracer, s_39_1);
        // S s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // C s_39_4: const #1u : u8
        let s_39_4: bool = true;
        // C s_39_5: cast zx s_39_4 -> bv
        let s_39_5: Bits = Bits::new(s_39_4 as u128, 1u16);
        // S s_39_6: cmp-eq s_39_3 s_39_5
        let s_39_6: bool = ((s_39_3) == (s_39_5));
        // D s_39_7: write-var gs#9284 <= s_39_6
        fn_state.gs_9284 = s_39_6;
        // N s_39_8: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#9284:u8
        let s_40_0: bool = fn_state.gs_9284;
        // N s_40_1: branch s_40_0 b46 b41
        if s_40_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var fault.15:struct
        let s_41_0: bool = fn_state.fault._15;
        // N s_41_1: branch s_41_0 b45 b42
        if s_41_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // D s_42_1: write-var gs#9285 <= s_42_0
        fn_state.gs_9285 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#9285:u8
        let s_43_0: bool = fn_state.gs_9285;
        // D s_43_1: write-var gs#9286 <= s_43_0
        fn_state.gs_9286 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#9286:u8
        let s_44_0: bool = fn_state.gs_9286;
        // D s_44_1: write-var gs#9287 <= s_44_0
        fn_state.gs_9287 = s_44_0;
        // N s_44_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #"Report abort using Long-descriptor format" : str
        let s_45_0: &'static str = "Report abort using Long-descriptor format";
        // S s_45_1: call __IMPDEF_boolean(s_45_0)
        let s_45_1: bool = u__IMPDEF_boolean(state, tracer, s_45_0);
        // D s_45_2: write-var gs#9285 <= s_45_1
        fn_state.gs_9285 = s_45_1;
        // N s_45_3: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #1u : u8
        let s_46_0: bool = true;
        // D s_46_1: write-var gs#9286 <= s_46_0
        fn_state.gs_9286 = s_46_0;
        // N s_46_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // D s_47_1: write-var gs#9284 <= s_47_0
        fn_state.gs_9284 = s_47_0;
        // N s_47_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #1u : u8
        let s_48_0: bool = true;
        // D s_48_1: write-var gs#9288 <= s_48_0
        fn_state.gs_9288 = s_48_0;
        // N s_48_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #() : ()
        let s_49_0: () = ();
        // S s_49_1: call CurrentSecurityState(s_49_0)
        let s_49_1: u32 = CurrentSecurityState(state, tracer, s_49_0);
        // C s_49_2: const #3u : u32
        let s_49_2: u32 = 3;
        // S s_49_3: cmp-eq s_49_1 s_49_2
        let s_49_3: bool = ((s_49_1) == (s_49_2));
        // D s_49_4: write-var gs#9282 <= s_49_3
        fn_state.gs_9282 = s_49_3;
        // N s_49_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
