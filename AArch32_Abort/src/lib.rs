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
use IsDebugException::*;
use AArch32_TakeDataAbortException::*;
use EL2Enabled::*;
use IsSecondStage::*;
use ELUsingAArch32::*;
use AArch32_TakePrefetchAbortException::*;
use AArch64_Abort::*;
use u_get_HCR_EL2_Type_TEA::*;
use EffectiveEA::*;
use u_get_HCR_EL2_Type_TGE::*;
use IsExternalAbort__1::*;
use HaveRASExt::*;
use u_get_MDCR_EL2_Type_TDE::*;
use common::*;
pub fn AArch32_Abort<T: Tracer>(
    state: &mut State,
    tracer: &T,
    vaddress: u32,
    fault: ProductType1d757adad216cdef,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_10005: bool,
        gs_9993: bool,
        gs_10003: bool,
        gs_10001: bool,
        gs_10000: bool,
        route_to_aarch64: bool,
        gs_9994: bool,
        gs_10004: bool,
        gs_9998: bool,
        gs_9995: bool,
        gs_9996: bool,
        ga_7316: ProductType9878976b5bcce9c9,
        gs_9992: bool,
        gs_10002: bool,
        vaddress: u32,
        fault: ProductType1d757adad216cdef,
    }
    let fn_state = FunctionState {
        vaddress,
        fault,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #448u : u32
        let s_0_3: u32 = 448;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-eq s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) == (s_0_5));
        // N s_0_7: branch s_0_6 b46 b1
        if s_0_6 {
            return block_46(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#9992 <= s_1_0
        fn_state.gs_9992 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#9992:u8
        let s_2_0: bool = fn_state.gs_9992;
        // D s_2_1: write-var route_to_aarch64 <= s_2_0
        fn_state.route_to_aarch64 = s_2_0;
        // D s_2_2: read-var route_to_aarch64:u8
        let s_2_2: bool = fn_state.route_to_aarch64;
        // D s_2_3: not s_2_2
        let s_2_3: bool = !s_2_2;
        // N s_2_4: branch s_2_3 b45 b3
        if s_2_3 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#9993 <= s_3_0
        fn_state.gs_9993 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#9993:u8
        let s_4_0: bool = fn_state.gs_9993;
        // N s_4_1: branch s_4_0 b44 b5
        if s_4_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#9994 <= s_5_0
        fn_state.gs_9994 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#9994:u8
        let s_6_0: bool = fn_state.gs_9994;
        // N s_6_1: branch s_6_0 b25 b7
        if s_6_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var route_to_aarch64:u8
        let s_8_0: bool = fn_state.route_to_aarch64;
        // D s_8_1: not s_8_0
        let s_8_1: bool = !s_8_0;
        // N s_8_2: branch s_8_1 b24 b9
        if s_8_1 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#9995 <= s_9_0
        fn_state.gs_9995 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#9995:u8
        let s_10_0: bool = fn_state.gs_9995;
        // N s_10_1: branch s_10_0 b23 b11
        if s_10_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#9996 <= s_11_0
        fn_state.gs_9996 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#9996:u8
        let s_12_0: bool = fn_state.gs_9996;
        // N s_12_1: branch s_12_0 b19 b13
        if s_12_0 {
            return block_19(state, tracer, fn_state);
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
        // D s_14_0: read-var route_to_aarch64:u8
        let s_14_0: bool = fn_state.route_to_aarch64;
        // N s_14_1: branch s_14_0 b18 b15
        if s_14_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var fault.0:struct
        let s_15_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_15_1: write-var ga#7316 <= s_15_0
        fn_state.ga_7316 = s_15_0;
        // D s_15_2: read-var ga#7316.1:struct
        let s_15_2: u32 = fn_state.ga_7316._1;
        // C s_15_3: const #0u : u32
        let s_15_3: u32 = 0;
        // D s_15_4: cmp-eq s_15_2 s_15_3
        let s_15_4: bool = ((s_15_2) == (s_15_3));
        // N s_15_5: branch s_15_4 b17 b16
        if s_15_4 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var vaddress:u32
        let s_16_0: u32 = fn_state.vaddress;
        // D s_16_1: read-var fault:struct
        let s_16_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_16_2: call AArch32_TakeDataAbortException(s_16_0, s_16_1)
        let s_16_2: () = AArch32_TakeDataAbortException(state, tracer, s_16_0, s_16_1);
        // N s_16_3: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var vaddress:u32
        let s_17_0: u32 = fn_state.vaddress;
        // D s_17_1: read-var fault:struct
        let s_17_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_17_2: call AArch32_TakePrefetchAbortException(s_17_0, s_17_1)
        let s_17_2: () = AArch32_TakePrefetchAbortException(
            state,
            tracer,
            s_17_0,
            s_17_1,
        );
        // N s_17_3: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #64s : i
        let s_18_0: i128 = 64;
        // D s_18_1: read-var vaddress:u32
        let s_18_1: u32 = fn_state.vaddress;
        // D s_18_2: cast zx s_18_1 -> bv
        let s_18_2: Bits = Bits::new(s_18_1 as u128, 32u16);
        // D s_18_3: bits-cast zx s_18_2 -> bv length s_18_0
        let s_18_3: Bits = s_18_2.zero_extend(s_18_0);
        // D s_18_4: cast reint s_18_3 -> u64
        let s_18_4: u64 = (s_18_3.value() as u64);
        // D s_18_5: read-var fault:struct
        let s_18_5: ProductType1d757adad216cdef = fn_state.fault;
        // D s_18_6: call AArch64_Abort(s_18_4, s_18_5)
        let s_18_6: () = AArch64_Abort(state, tracer, s_18_4, s_18_5);
        // N s_18_7: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call EffectiveEA(s_19_0)
        let s_19_1: bool = EffectiveEA(state, tracer, s_19_0);
        // S s_19_2: cast zx s_19_1 -> bv
        let s_19_2: Bits = Bits::new(s_19_1 as u128, 1u16);
        // C s_19_3: const #1u : u8
        let s_19_3: bool = true;
        // C s_19_4: cast zx s_19_3 -> bv
        let s_19_4: Bits = Bits::new(s_19_3 as u128, 1u16);
        // S s_19_5: cmp-eq s_19_2 s_19_4
        let s_19_5: bool = ((s_19_2) == (s_19_4));
        // N s_19_6: branch s_19_5 b22 b20
        if s_19_5 {
            return block_22(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#9998 <= s_20_0
        fn_state.gs_9998 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#9998:u8
        let s_21_0: bool = fn_state.gs_9998;
        // D s_21_1: write-var route_to_aarch64 <= s_21_0
        fn_state.route_to_aarch64 = s_21_0;
        // N s_21_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var fault:struct
        let s_22_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_22_1: call IsExternalAbort__1(s_22_0)
        let s_22_1: bool = IsExternalAbort__1(state, tracer, s_22_0);
        // D s_22_2: write-var gs#9998 <= s_22_1
        fn_state.gs_9998 = s_22_1;
        // N s_22_3: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #424u : u32
        let s_23_0: u32 = 424;
        // D s_23_1: read-reg s_23_0:u8
        let s_23_1: u8 = {
            let value = state.read_register::<u8>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call ELUsingAArch32(s_23_1)
        let s_23_2: bool = ELUsingAArch32(state, tracer, s_23_1);
        // D s_23_3: not s_23_2
        let s_23_3: bool = !s_23_2;
        // D s_23_4: write-var gs#9996 <= s_23_3
        fn_state.gs_9996 = s_23_3;
        // N s_23_5: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #424u : u32
        let s_24_0: u32 = 424;
        // D s_24_1: read-reg s_24_0:u8
        let s_24_1: u8 = {
            let value = state.read_register::<u8>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // C s_24_2: const #2u : u8
        let s_24_2: u8 = 2;
        // D s_24_3: cmp-lt s_24_1 s_24_2
        let s_24_3: bool = ((s_24_1) < (s_24_2));
        // D s_24_4: write-var gs#9995 <= s_24_3
        fn_state.gs_9995 = s_24_3;
        // N s_24_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #102552u : u32
        let s_25_0: u32 = 102552;
        // D s_25_1: read-reg s_25_0:struct
        let s_25_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: call _get_HCR_EL2_Type_TGE(s_25_1)
        let s_25_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_25_1);
        // D s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // C s_25_4: const #1u : u8
        let s_25_4: bool = true;
        // C s_25_5: cast zx s_25_4 -> bv
        let s_25_5: Bits = Bits::new(s_25_4 as u128, 1u16);
        // D s_25_6: cmp-eq s_25_3 s_25_5
        let s_25_6: bool = ((s_25_3) == (s_25_5));
        // N s_25_7: branch s_25_6 b43 b26
        if s_25_6 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var fault:struct
        let s_26_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_26_1: call IsSecondStage(s_26_0)
        let s_26_1: bool = IsSecondStage(state, tracer, s_26_0);
        // D s_26_2: write-var gs#10000 <= s_26_1
        fn_state.gs_10000 = s_26_1;
        // N s_26_3: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#10000:u8
        let s_27_0: bool = fn_state.gs_10000;
        // N s_27_1: branch s_27_0 b42 b28
        if s_27_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call HaveRASExt(s_28_0)
        let s_28_1: bool = HaveRASExt(state, tracer, s_28_0);
        // N s_28_2: branch s_28_1 b41 b29
        if s_28_1 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#10001 <= s_29_0
        fn_state.gs_10001 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#10001:u8
        let s_30_0: bool = fn_state.gs_10001;
        // N s_30_1: branch s_30_0 b40 b31
        if s_30_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var gs#10002 <= s_31_0
        fn_state.gs_10002 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#10002:u8
        let s_32_0: bool = fn_state.gs_10002;
        // D s_32_1: write-var gs#10003 <= s_32_0
        fn_state.gs_10003 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#10003:u8
        let s_33_0: bool = fn_state.gs_10003;
        // N s_33_1: branch s_33_0 b39 b34
        if s_33_0 {
            return block_39(state, tracer, fn_state);
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
        // D s_34_1: call IsDebugException(s_34_0)
        let s_34_1: bool = IsDebugException(state, tracer, s_34_0);
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
        // D s_35_1: write-var gs#10004 <= s_35_0
        fn_state.gs_10004 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#10004:u8
        let s_36_0: bool = fn_state.gs_10004;
        // D s_36_1: write-var gs#10005 <= s_36_0
        fn_state.gs_10005 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#10005:u8
        let s_37_0: bool = fn_state.gs_10005;
        // D s_37_1: write-var route_to_aarch64 <= s_37_0
        fn_state.route_to_aarch64 = s_37_0;
        // N s_37_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #104880u : u32
        let s_38_0: u32 = 104880;
        // D s_38_1: read-reg s_38_0:struct
        let s_38_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // D s_38_2: call _get_MDCR_EL2_Type_TDE(s_38_1)
        let s_38_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_38_1);
        // D s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 1u16);
        // C s_38_4: const #1u : u8
        let s_38_4: bool = true;
        // C s_38_5: cast zx s_38_4 -> bv
        let s_38_5: Bits = Bits::new(s_38_4 as u128, 1u16);
        // D s_38_6: cmp-eq s_38_3 s_38_5
        let s_38_6: bool = ((s_38_3) == (s_38_5));
        // D s_38_7: write-var gs#10004 <= s_38_6
        fn_state.gs_10004 = s_38_6;
        // N s_38_8: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var gs#10005 <= s_39_0
        fn_state.gs_10005 = s_39_0;
        // N s_39_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var fault:struct
        let s_40_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_40_1: call IsExternalAbort__1(s_40_0)
        let s_40_1: bool = IsExternalAbort__1(state, tracer, s_40_0);
        // D s_40_2: write-var gs#10002 <= s_40_1
        fn_state.gs_10002 = s_40_1;
        // N s_40_3: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #102552u : u32
        let s_41_0: u32 = 102552;
        // D s_41_1: read-reg s_41_0:struct
        let s_41_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_41_0 as isize);
            tracer.read_register(s_41_0 as isize, value);
            value
        };
        // D s_41_2: call _get_HCR_EL2_Type_TEA(s_41_1)
        let s_41_2: bool = u_get_HCR_EL2_Type_TEA(state, tracer, s_41_1);
        // D s_41_3: cast zx s_41_2 -> bv
        let s_41_3: Bits = Bits::new(s_41_2 as u128, 1u16);
        // C s_41_4: const #1u : u8
        let s_41_4: bool = true;
        // C s_41_5: cast zx s_41_4 -> bv
        let s_41_5: Bits = Bits::new(s_41_4 as u128, 1u16);
        // D s_41_6: cmp-eq s_41_3 s_41_5
        let s_41_6: bool = ((s_41_3) == (s_41_5));
        // D s_41_7: write-var gs#10001 <= s_41_6
        fn_state.gs_10001 = s_41_6;
        // N s_41_8: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // D s_42_1: write-var gs#10003 <= s_42_0
        fn_state.gs_10003 = s_42_0;
        // N s_42_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #1u : u8
        let s_43_0: bool = true;
        // D s_43_1: write-var gs#10000 <= s_43_0
        fn_state.gs_10000 = s_43_0;
        // N s_43_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #432u : u32
        let s_44_0: u32 = 432;
        // D s_44_1: read-reg s_44_0:u8
        let s_44_1: u8 = {
            let value = state.read_register::<u8>(s_44_0 as isize);
            tracer.read_register(s_44_0 as isize, value);
            value
        };
        // D s_44_2: call ELUsingAArch32(s_44_1)
        let s_44_2: bool = ELUsingAArch32(state, tracer, s_44_1);
        // D s_44_3: not s_44_2
        let s_44_3: bool = !s_44_2;
        // D s_44_4: write-var gs#9994 <= s_44_3
        fn_state.gs_9994 = s_44_3;
        // N s_44_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call EL2Enabled(s_45_0)
        let s_45_1: bool = EL2Enabled(state, tracer, s_45_0);
        // D s_45_2: write-var gs#9993 <= s_45_1
        fn_state.gs_9993 = s_45_1;
        // N s_45_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #440u : u32
        let s_46_0: u32 = 440;
        // D s_46_1: read-reg s_46_0:u8
        let s_46_1: u8 = {
            let value = state.read_register::<u8>(s_46_0 as isize);
            tracer.read_register(s_46_0 as isize, value);
            value
        };
        // D s_46_2: call ELUsingAArch32(s_46_1)
        let s_46_2: bool = ELUsingAArch32(state, tracer, s_46_1);
        // D s_46_3: not s_46_2
        let s_46_3: bool = !s_46_2;
        // D s_46_4: write-var gs#9992 <= s_46_3
        fn_state.gs_9992 = s_46_3;
        // N s_46_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
