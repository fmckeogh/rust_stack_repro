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
use IsFault__2::*;
use FailTransaction::*;
use HaveMTE2Ext::*;
use AArch64_PhysicalTag::*;
use AArch64_TagCheckFault::*;
use u__IMPDEF_boolean::*;
use Zeros::*;
use MemHasTransactionalAccess::*;
use HaveTME::*;
use PhysMemWrite::*;
use AArch64_TranslateAddress::*;
use AArch64_AccessIsTagChecked::*;
use HandleExternalWriteAbort::*;
use AArch64_AllocationTagAccessIsEnabled::*;
use AArch64_Abort::*;
use IsFault::*;
use AArch64_CheckTag::*;
use common::*;
pub fn AArch64_DataMemZero<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regval: u64,
    vaddress: u64,
    accdesc_in: ProductType9878976b5bcce9c9,
    size: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_20969: bool,
        memstatus: ProductTypef8c3639b88223255,
        ga_16390: ProductTypeda0231e9dc169f81,
        accdesc: ProductType9878976b5bcce9c9,
        gs_20981: bool,
        gs_20960: bool,
        memaddrdesc: ProductTypece7c66ccb2cab13e,
        i: i64,
        ga_16384: bool,
        gs_20966: i64,
        gs_20959: bool,
        regval: u64,
        vaddress: u64,
        accdesc_in: ProductType9878976b5bcce9c9,
        size: i128,
    }
    let fn_state = FunctionState {
        regval,
        vaddress,
        accdesc_in,
        size,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var accdesc_in:struct
        let s_0_0: ProductType9878976b5bcce9c9 = fn_state.accdesc_in;
        // D s_0_1: write-var accdesc <= s_0_0
        fn_state.accdesc = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call HaveMTE2Ext(s_0_2)
        let s_0_3: bool = HaveMTE2Ext(state, tracer, s_0_2);
        // N s_0_4: branch s_0_3 b45 b1
        if s_0_3 {
            return block_45(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#20959 <= s_1_0
        fn_state.gs_20959 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#20959:u8
        let s_2_0: bool = fn_state.gs_20959;
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
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call HaveMTE2Ext(s_4_0)
        let s_4_1: bool = HaveMTE2Ext(state, tracer, s_4_0);
        // N s_4_2: branch s_4_1 b43 b5
        if s_4_1 {
            return block_43(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#20960 <= s_5_0
        fn_state.gs_20960 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#20960:u8
        let s_6_0: bool = fn_state.gs_20960;
        // N s_6_1: branch s_6_0 b42 b7
        if s_6_0 {
            return block_42(state, tracer, fn_state);
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
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // D s_8_1: read-var vaddress:u64
        let s_8_1: u64 = fn_state.vaddress;
        // D s_8_2: read-var accdesc:struct
        let s_8_2: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_8_3: read-var size:i
        let s_8_3: i128 = fn_state.size;
        // D s_8_4: call AArch64_TranslateAddress(s_8_1, s_8_2, s_8_0, s_8_3)
        let s_8_4: ProductTypece7c66ccb2cab13e = AArch64_TranslateAddress(
            state,
            tracer,
            s_8_1,
            s_8_2,
            s_8_0,
            s_8_3,
        );
        // D s_8_5: write-var memaddrdesc <= s_8_4
        fn_state.memaddrdesc = s_8_4;
        // N s_8_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var memaddrdesc:struct
        let s_9_0: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_9_1: call IsFault(s_9_0)
        let s_9_1: bool = IsFault(state, tracer, s_9_0);
        // N s_9_2: branch s_9_1 b39 b10
        if s_9_1 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call HaveTME(s_11_0)
        let s_11_1: bool = HaveTME(state, tracer, s_11_0);
        // N s_11_2: branch s_11_1 b32 b12
        if s_11_1 {
            return block_32(state, tracer, fn_state);
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
        // C s_13_0: const #0s : i64
        let s_13_0: i64 = 0;
        // C s_13_1: const #1s : i
        let s_13_1: i128 = 1;
        // D s_13_2: read-var size:i
        let s_13_2: i128 = fn_state.size;
        // D s_13_3: sub s_13_2 s_13_1
        let s_13_3: i128 = ((s_13_2) - (s_13_1));
        // D s_13_4: cast reint s_13_3 -> i64
        let s_13_4: i64 = (s_13_3 as i64);
        // D s_13_5: write-var gs#20966 <= s_13_4
        fn_state.gs_20966 = s_13_4;
        // D s_13_6: write-var i <= s_13_0
        fn_state.i = s_13_0;
        // N s_13_7: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var i:i64
        let s_14_0: i64 = fn_state.i;
        // D s_14_1: read-var gs#20966:i64
        let s_14_1: i64 = fn_state.gs_20966;
        // D s_14_2: cmp-gt s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) > (s_14_1));
        // N s_14_3: branch s_14_2 b31 b15
        if s_14_2 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call HaveMTE2Ext(s_15_0)
        let s_15_1: bool = HaveMTE2Ext(state, tracer, s_15_0);
        // N s_15_2: branch s_15_1 b30 b16
        if s_15_1 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#20969 <= s_16_0
        fn_state.gs_20969 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#20969:u8
        let s_17_0: bool = fn_state.gs_20969;
        // N s_17_1: branch s_17_0 b23 b18
        if s_17_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // D s_19_1: create-sum enum = 0:"s_19_0"
        let s_19_1: SumTypeb20592b6489a79bd = SumTypeb20592b6489a79bd::_0(s_19_0);
        // C s_19_2: const #8s : i
        let s_19_2: i128 = 8;
        // S s_19_3: call Zeros(s_19_2)
        let s_19_3: Bits = Zeros(state, tracer, s_19_2);
        // S s_19_4: cast reint s_19_3 -> u8
        let s_19_4: u8 = (s_19_3.value() as u8);
        // C s_19_5: const #1s : i
        let s_19_5: i128 = 1;
        // S s_19_6: cast zx s_19_4 -> bv
        let s_19_6: Bits = Bits::new(s_19_4 as u128, 8u16);
        // D s_19_7: read-var memaddrdesc:struct
        let s_19_7: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_19_8: read-var accdesc:struct
        let s_19_8: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_19_9: call PhysMemWrite(s_19_7, s_19_5, s_19_8, s_19_1, s_19_6)
        let s_19_9: ProductTypef8c3639b88223255 = PhysMemWrite(
            state,
            tracer,
            s_19_7,
            s_19_5,
            s_19_8,
            s_19_1,
            s_19_6,
        );
        // D s_19_10: write-var memstatus <= s_19_9
        fn_state.memstatus = s_19_9;
        // D s_19_11: read-var memstatus:struct
        let s_19_11: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_19_12: call IsFault__2(s_19_11)
        let s_19_12: bool = IsFault__2(state, tracer, s_19_11);
        // N s_19_13: branch s_19_12 b22 b20
        if s_19_12 {
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
        // N s_20_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var memaddrdesc.3:struct
        let s_21_0: ProductTypeda0231e9dc169f81 = fn_state.memaddrdesc._3;
        // D s_21_1: write-var ga#16390 <= s_21_0
        fn_state.ga_16390 = s_21_0;
        // D s_21_2: read-var ga#16390.0:struct
        let s_21_2: u64 = fn_state.ga_16390._0;
        // C s_21_3: const #1s : i
        let s_21_3: i128 = 1;
        // D s_21_4: cast zx s_21_2 -> bv
        let s_21_4: Bits = Bits::new(s_21_2 as u128, 56u16);
        // C s_21_5: cast cvt s_21_3 -> bv
        let s_21_5: Bits = Bits::new(s_21_3 as u128, 128);
        // D s_21_6: add s_21_4 s_21_5
        let s_21_6: Bits = (s_21_4 + s_21_5);
        // D s_21_7: cast reint s_21_6 -> u56
        let s_21_7: u64 = (s_21_6.value() as u64);
        // D s_21_8: write-var memaddrdesc.3.0 <= s_21_7
        fn_state.memaddrdesc._3._0 = s_21_7;
        // D s_21_9: read-var i:i64
        let s_21_9: i64 = fn_state.i;
        // C s_21_10: const #1s : i64
        let s_21_10: i64 = 1;
        // D s_21_11: add s_21_9 s_21_10
        let s_21_11: i64 = (s_21_9 + s_21_10);
        // D s_21_12: write-var i <= s_21_11
        fn_state.i = s_21_11;
        // N s_21_13: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1s : i
        let s_22_0: i128 = 1;
        // D s_22_1: read-var memstatus:struct
        let s_22_1: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_22_2: read-var memaddrdesc:struct
        let s_22_2: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_22_3: read-var accdesc:struct
        let s_22_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_22_4: call HandleExternalWriteAbort(s_22_1, s_22_2, s_22_0, s_22_3)
        let s_22_4: () = HandleExternalWriteAbort(
            state,
            tracer,
            s_22_1,
            s_22_2,
            s_22_0,
            s_22_3,
        );
        // N s_22_5: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var vaddress:u64
        let s_23_0: u64 = fn_state.vaddress;
        // D s_23_1: call AArch64_PhysicalTag(s_23_0)
        let s_23_1: u8 = AArch64_PhysicalTag(state, tracer, s_23_0);
        // D s_23_2: read-var memaddrdesc:struct
        let s_23_2: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_23_3: read-var accdesc:struct
        let s_23_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_23_4: call AArch64_CheckTag(s_23_2, s_23_3, s_23_1)
        let s_23_4: bool = AArch64_CheckTag(state, tracer, s_23_2, s_23_3, s_23_1);
        // D s_23_5: write-var ga#16384 <= s_23_4
        fn_state.ga_16384 = s_23_4;
        // N s_23_6: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var ga#16384:u8
        let s_24_0: bool = fn_state.ga_16384;
        // D s_24_1: not s_24_0
        let s_24_1: bool = !s_24_0;
        // N s_24_2: branch s_24_1 b27 b25
        if s_24_1 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #"DC_ZVA tag fault reported with lowest faulting address" : str
        let s_27_0: &'static str = "DC_ZVA tag fault reported with lowest faulting address";
        // S s_27_1: call __IMPDEF_boolean(s_27_0)
        let s_27_1: bool = u__IMPDEF_boolean(state, tracer, s_27_0);
        // N s_27_2: branch s_27_1 b29 b28
        if s_27_1 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var regval:u64
        let s_28_0: u64 = fn_state.regval;
        // D s_28_1: read-var accdesc:struct
        let s_28_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_28_2: call AArch64_TagCheckFault(s_28_0, s_28_1)
        let s_28_2: () = AArch64_TagCheckFault(state, tracer, s_28_0, s_28_1);
        // N s_28_3: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var vaddress:u64
        let s_29_0: u64 = fn_state.vaddress;
        // D s_29_1: read-var accdesc:struct
        let s_29_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_29_2: call AArch64_TagCheckFault(s_29_0, s_29_1)
        let s_29_2: () = AArch64_TagCheckFault(state, tracer, s_29_0, s_29_1);
        // N s_29_3: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var accdesc.28:struct
        let s_30_0: bool = fn_state.accdesc._28;
        // D s_30_1: write-var gs#20969 <= s_30_0
        fn_state.gs_20969 = s_30_0;
        // N s_30_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_31_0: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var accdesc.30:struct
        let s_32_0: bool = fn_state.accdesc._30;
        // N s_32_1: branch s_32_0 b38 b33
        if s_32_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var gs#20981 <= s_33_0
        fn_state.gs_20981 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#20981:u8
        let s_34_0: bool = fn_state.gs_20981;
        // N s_34_1: branch s_34_0 b37 b35
        if s_34_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_36_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #7u : u32
        let s_37_0: u32 = 7;
        // C s_37_1: const #0u : u8
        let s_37_1: bool = false;
        // S s_37_2: call FailTransaction(s_37_0, s_37_1)
        let s_37_2: () = FailTransaction(state, tracer, s_37_0, s_37_1);
        // N s_37_3: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var memaddrdesc.2:struct
        let s_38_0: ProductTypef170cab34335b70c = fn_state.memaddrdesc._2;
        // D s_38_1: call MemHasTransactionalAccess(s_38_0)
        let s_38_1: bool = MemHasTransactionalAccess(state, tracer, s_38_0);
        // D s_38_2: not s_38_1
        let s_38_2: bool = !s_38_1;
        // D s_38_3: write-var gs#20981 <= s_38_2
        fn_state.gs_20981 = s_38_2;
        // N s_38_4: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var memaddrdesc.0:struct
        let s_39_0: ProductType1d757adad216cdef = fn_state.memaddrdesc._0;
        // D s_39_1: call IsDebugException(s_39_0)
        let s_39_1: bool = IsDebugException(state, tracer, s_39_0);
        // N s_39_2: branch s_39_1 b41 b40
        if s_39_1 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var memaddrdesc.0:struct
        let s_40_0: ProductType1d757adad216cdef = fn_state.memaddrdesc._0;
        // D s_40_1: read-var regval:u64
        let s_40_1: u64 = fn_state.regval;
        // D s_40_2: call AArch64_Abort(s_40_1, s_40_0)
        let s_40_2: () = AArch64_Abort(state, tracer, s_40_1, s_40_0);
        // N s_40_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var memaddrdesc.0:struct
        let s_41_0: ProductType1d757adad216cdef = fn_state.memaddrdesc._0;
        // D s_41_1: read-var vaddress:u64
        let s_41_1: u64 = fn_state.vaddress;
        // D s_41_2: call AArch64_Abort(s_41_1, s_41_0)
        let s_41_2: () = AArch64_Abort(state, tracer, s_41_1, s_41_0);
        // N s_41_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var vaddress:u64
        let s_42_0: u64 = fn_state.vaddress;
        // D s_42_1: read-var accdesc:struct
        let s_42_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_42_2: call AArch64_AccessIsTagChecked(s_42_0, s_42_1)
        let s_42_2: bool = AArch64_AccessIsTagChecked(state, tracer, s_42_0, s_42_1);
        // D s_42_3: write-var accdesc.28 <= s_42_2
        fn_state.accdesc._28 = s_42_2;
        // N s_42_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var accdesc.28:struct
        let s_43_0: bool = fn_state.accdesc._28;
        // D s_43_1: write-var gs#20960 <= s_43_0
        fn_state.gs_20960 = s_43_0;
        // N s_43_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var accdesc.8:struct
        let s_44_0: u8 = fn_state.accdesc._8;
        // D s_44_1: call AArch64_AllocationTagAccessIsEnabled(s_44_0)
        let s_44_1: bool = AArch64_AllocationTagAccessIsEnabled(state, tracer, s_44_0);
        // D s_44_2: write-var accdesc.27 <= s_44_1
        fn_state.accdesc._27 = s_44_1;
        // N s_44_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var accdesc.27:struct
        let s_45_0: bool = fn_state.accdesc._27;
        // D s_45_1: write-var gs#20959 <= s_45_0
        fn_state.gs_20959 = s_45_0;
        // N s_45_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
