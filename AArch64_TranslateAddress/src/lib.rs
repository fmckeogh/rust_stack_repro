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
use GranuleProtectionCheck::*;
use HaveRME::*;
use IsFault::*;
use AArch64_CheckDebug::*;
use SPEStopCounter::*;
use u__IMPDEF_boolean::*;
use SPEStartCounter::*;
use AArch64_FullTranslate::*;
use common::*;
pub fn AArch64_TranslateAddress<T: Tracer>(
    state: &mut State,
    tracer: &T,
    va: u64,
    accdesc: ProductType9878976b5bcce9c9,
    aligned: bool,
    size: i128,
) -> ProductTypece7c66ccb2cab13e {
    #[derive(Default)]
    struct FunctionState {
        gs_20006: bool,
        gs_20004: bool,
        gs_20008: bool,
        ga_15427: ProductType396b95aa74979079,
        gs_20009: bool,
        gs_20002: bool,
        result: ProductTypece7c66ccb2cab13e,
        gs_20001: bool,
        gs_20007: bool,
        gs_20005: bool,
        gs_20010: bool,
        ga_15426: ProductType1d757adad216cdef,
        va: u64,
        accdesc: ProductType9878976b5bcce9c9,
        aligned: bool,
        size: i128,
    }
    let fn_state = FunctionState {
        va,
        accdesc,
        aligned,
        size,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_0_0: const #22416u : u32
        let s_0_0: u32 = 22416;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: bool = {
            let value = state.read_register::<bool>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // N s_0_2: branch s_0_1 b44 b1
        if s_0_1 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#20002 <= s_1_0
        fn_state.gs_20002 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_2_0: read-var gs#20002:u8
        let s_2_0: bool = fn_state.gs_20002;
        // N s_2_1: branch s_2_0 b43 b3
        if s_2_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_4_0: read-var va:u64
        let s_4_0: u64 = fn_state.va;
        // D s_4_1: read-var accdesc:struct
        let s_4_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_4_2: read-var aligned:u8
        let s_4_2: bool = fn_state.aligned;
        // D s_4_3: call AArch64_FullTranslate(s_4_0, s_4_1, s_4_2)
        let s_4_3: ProductTypece7c66ccb2cab13e = AArch64_FullTranslate(
            state,
            tracer,
            s_4_0,
            s_4_1,
            s_4_2,
        );
        // D s_4_4: write-var result <= s_4_3
        fn_state.result = s_4_3;
        // D s_4_5: read-var result:struct
        let s_4_5: ProductTypece7c66ccb2cab13e = fn_state.result;
        // D s_4_6: call IsFault(s_4_5)
        let s_4_6: bool = IsFault(state, tracer, s_4_5);
        // D s_4_7: not s_4_6
        let s_4_7: bool = !s_4_6;
        // N s_4_8: branch s_4_7 b42 b5
        if s_4_7 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#20004 <= s_5_0
        fn_state.gs_20004 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_6_0: read-var gs#20004:u8
        let s_6_0: bool = fn_state.gs_20004;
        // N s_6_1: branch s_6_0 b40 b7
        if s_6_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call HaveRME(s_8_0)
        let s_8_1: bool = HaveRME(state, tracer, s_8_0);
        // N s_8_2: branch s_8_1 b39 b9
        if s_8_1 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#20005 <= s_9_0
        fn_state.gs_20005 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_10_0: read-var gs#20005:u8
        let s_10_0: bool = fn_state.gs_20005;
        // N s_10_1: branch s_10_0 b35 b11
        if s_10_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#20007 <= s_11_0
        fn_state.gs_20007 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_12_0: read-var gs#20007:u8
        let s_12_0: bool = fn_state.gs_20007;
        // N s_12_1: branch s_12_0 b31 b13
        if s_12_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_14_0: read-var result:struct
        let s_14_0: ProductTypece7c66ccb2cab13e = fn_state.result;
        // D s_14_1: call IsFault(s_14_0)
        let s_14_1: bool = IsFault(state, tracer, s_14_0);
        // D s_14_2: not s_14_1
        let s_14_2: bool = !s_14_1;
        // N s_14_3: branch s_14_2 b30 b15
        if s_14_2 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#20008 <= s_15_0
        fn_state.gs_20008 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_16_0: read-var gs#20008:u8
        let s_16_0: bool = fn_state.gs_20008;
        // N s_16_1: branch s_16_0 b28 b17
        if s_16_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // N s_17_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_18_0: const #22416u : u32
        let s_18_0: u32 = 22416;
        // D s_18_1: read-reg s_18_0:u8
        let s_18_1: bool = {
            let value = state.read_register::<bool>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // N s_18_2: branch s_18_1 b24 b19
        if s_18_1 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#20010 <= s_19_0
        fn_state.gs_20010 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_20_0: read-var gs#20010:u8
        let s_20_0: bool = fn_state.gs_20010;
        // N s_20_1: branch s_20_0 b23 b21
        if s_20_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // N s_21_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_22_0: const #64s : i
        let s_22_0: i128 = 64;
        // D s_22_1: read-var va:u64
        let s_22_1: u64 = fn_state.va;
        // D s_22_2: cast zx s_22_1 -> bv
        let s_22_2: Bits = Bits::new(s_22_1 as u128, 64u16);
        // D s_22_3: bits-cast zx s_22_2 -> bv length s_22_0
        let s_22_3: Bits = s_22_2.zero_extend(s_22_0);
        // D s_22_4: cast reint s_22_3 -> u64
        let s_22_4: u64 = (s_22_3.value() as u64);
        // D s_22_5: write-var result.7 <= s_22_4
        fn_state.result._7 = s_22_4;
        // D s_22_6: read-var result:struct
        let s_22_6: ProductTypece7c66ccb2cab13e = fn_state.result;
        // N s_22_7: return s_22_6
        return s_22_6;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_23_0: const #1072u : u32
        let s_23_0: u32 = 1072;
        // D s_23_1: read-reg s_23_0:i64
        let s_23_1: i64 = {
            let value = state.read_register::<i64>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call SPEStopCounter(s_23_1)
        let s_23_2: () = SPEStopCounter(state, tracer, s_23_1);
        // N s_23_3: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_24_0: read-var accdesc.1:struct
        let s_24_0: u32 = fn_state.accdesc._1;
        // C s_24_1: const #0u : u32
        let s_24_1: u32 = 0;
        // D s_24_2: cmp-eq s_24_0 s_24_1
        let s_24_2: bool = ((s_24_0) == (s_24_1));
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
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_25_0: read-var accdesc.1:struct
        let s_25_0: u32 = fn_state.accdesc._1;
        // C s_25_1: const #10u : u32
        let s_25_1: u32 = 10;
        // D s_25_2: cmp-eq s_25_0 s_25_1
        let s_25_2: bool = ((s_25_0) == (s_25_1));
        // D s_25_3: write-var gs#20009 <= s_25_2
        fn_state.gs_20009 = s_25_2;
        // N s_25_4: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_26_0: read-var gs#20009:u8
        let s_26_0: bool = fn_state.gs_20009;
        // D s_26_1: not s_26_0
        let s_26_1: bool = !s_26_0;
        // D s_26_2: write-var gs#20010 <= s_26_1
        fn_state.gs_20010 = s_26_1;
        // N s_26_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#20009 <= s_27_0
        fn_state.gs_20009 = s_27_0;
        // N s_27_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_28_0: read-var va:u64
        let s_28_0: u64 = fn_state.va;
        // D s_28_1: read-var accdesc:struct
        let s_28_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_28_2: read-var size:i
        let s_28_2: i128 = fn_state.size;
        // D s_28_3: call AArch64_CheckDebug(s_28_0, s_28_1, s_28_2)
        let s_28_3: ProductType1d757adad216cdef = AArch64_CheckDebug(
            state,
            tracer,
            s_28_0,
            s_28_1,
            s_28_2,
        );
        // D s_28_4: write-var result.0 <= s_28_3
        fn_state.result._0 = s_28_3;
        // N s_28_5: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // N s_29_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_30_0: read-var accdesc.1:struct
        let s_30_0: u32 = fn_state.accdesc._1;
        // C s_30_1: const #0u : u32
        let s_30_1: u32 = 0;
        // D s_30_2: cmp-eq s_30_0 s_30_1
        let s_30_2: bool = ((s_30_0) == (s_30_1));
        // D s_30_3: write-var gs#20008 <= s_30_2
        fn_state.gs_20008 = s_30_2;
        // N s_30_4: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_31_0: read-var result:struct
        let s_31_0: ProductTypece7c66ccb2cab13e = fn_state.result;
        // D s_31_1: read-var accdesc:struct
        let s_31_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_31_2: call GranuleProtectionCheck(s_31_0, s_31_1)
        let s_31_2: ProductType396b95aa74979079 = GranuleProtectionCheck(
            state,
            tracer,
            s_31_0,
            s_31_1,
        );
        // D s_31_3: write-var result.0.6 <= s_31_2
        fn_state.result._0._6 = s_31_2;
        // D s_31_4: read-var result.0:struct
        let s_31_4: ProductType1d757adad216cdef = fn_state.result._0;
        // D s_31_5: write-var ga#15426 <= s_31_4
        fn_state.ga_15426 = s_31_4;
        // D s_31_6: read-var ga#15426.6:struct
        let s_31_6: ProductType396b95aa74979079 = fn_state.ga_15426._6;
        // D s_31_7: write-var ga#15427 <= s_31_6
        fn_state.ga_15427 = s_31_6;
        // D s_31_8: read-var ga#15427.0:struct
        let s_31_8: u32 = fn_state.ga_15427._0;
        // C s_31_9: const #0u : u32
        let s_31_9: u32 = 0;
        // D s_31_10: cmp-eq s_31_8 s_31_9
        let s_31_10: bool = ((s_31_8) == (s_31_9));
        // N s_31_11: branch s_31_10 b34 b32
        if s_31_10 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // N s_32_0: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // N s_33_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_34_0: const #13u : u32
        let s_34_0: u32 = 13;
        // D s_34_1: write-var result.0.16 <= s_34_0
        fn_state.result._0._16 = s_34_0;
        // D s_34_2: read-var result.3:struct
        let s_34_2: ProductTypeda0231e9dc169f81 = fn_state.result._3;
        // D s_34_3: write-var result.0.12 <= s_34_2
        fn_state.result._0._12 = s_34_2;
        // N s_34_4: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_35_0: read-var accdesc.1:struct
        let s_35_0: u32 = fn_state.accdesc._1;
        // C s_35_1: const #6u : u32
        let s_35_1: u32 = 6;
        // D s_35_2: cmp-eq s_35_0 s_35_1
        let s_35_2: bool = ((s_35_0) == (s_35_1));
        // N s_35_3: branch s_35_2 b38 b36
        if s_35_2 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_36_0: const #"GPC Fault on DC operations" : str
        let s_36_0: &'static str = "GPC Fault on DC operations";
        // S s_36_1: call __IMPDEF_boolean(s_36_0)
        let s_36_1: bool = u__IMPDEF_boolean(state, tracer, s_36_0);
        // D s_36_2: write-var gs#20006 <= s_36_1
        fn_state.gs_20006 = s_36_1;
        // N s_36_3: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_37_0: read-var gs#20006:u8
        let s_37_0: bool = fn_state.gs_20006;
        // D s_37_1: write-var gs#20007 <= s_37_0
        fn_state.gs_20007 = s_37_0;
        // N s_37_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_38_0: const #1u : u8
        let s_38_0: bool = true;
        // D s_38_1: write-var gs#20006 <= s_38_0
        fn_state.gs_20006 = s_38_0;
        // N s_38_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_39_0: read-var result:struct
        let s_39_0: ProductTypece7c66ccb2cab13e = fn_state.result;
        // D s_39_1: call IsFault(s_39_0)
        let s_39_1: bool = IsFault(state, tracer, s_39_0);
        // D s_39_2: not s_39_1
        let s_39_2: bool = !s_39_1;
        // D s_39_3: write-var gs#20005 <= s_39_2
        fn_state.gs_20005 = s_39_2;
        // N s_39_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_40_0: read-var va:u64
        let s_40_0: u64 = fn_state.va;
        // D s_40_1: read-var accdesc:struct
        let s_40_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_40_2: read-var size:i
        let s_40_2: i128 = fn_state.size;
        // D s_40_3: call AArch64_CheckDebug(s_40_0, s_40_1, s_40_2)
        let s_40_3: ProductType1d757adad216cdef = AArch64_CheckDebug(
            state,
            tracer,
            s_40_0,
            s_40_1,
            s_40_2,
        );
        // D s_40_4: write-var result.0 <= s_40_3
        fn_state.result._0 = s_40_3;
        // N s_40_5: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // N s_41_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_42_0: read-var accdesc.1:struct
        let s_42_0: u32 = fn_state.accdesc._1;
        // C s_42_1: const #0u : u32
        let s_42_1: u32 = 0;
        // D s_42_2: cmp-eq s_42_0 s_42_1
        let s_42_2: bool = ((s_42_0) == (s_42_1));
        // D s_42_3: write-var gs#20004 <= s_42_2
        fn_state.gs_20004 = s_42_2;
        // N s_42_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_43_0: const #1072u : u32
        let s_43_0: u32 = 1072;
        // D s_43_1: read-reg s_43_0:i64
        let s_43_1: i64 = {
            let value = state.read_register::<i64>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: cast zx s_43_1 -> i
        let s_43_2: i128 = (i128::try_from(s_43_1).unwrap());
        // D s_43_3: call SPEStartCounter(s_43_2)
        let s_43_3: () = SPEStartCounter(state, tracer, s_43_2);
        // N s_43_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_44_0: read-var accdesc.1:struct
        let s_44_0: u32 = fn_state.accdesc._1;
        // C s_44_1: const #0u : u32
        let s_44_1: u32 = 0;
        // D s_44_2: cmp-eq s_44_0 s_44_1
        let s_44_2: bool = ((s_44_0) == (s_44_1));
        // N s_44_3: branch s_44_2 b47 b45
        if s_44_2 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_45_0: read-var accdesc.1:struct
        let s_45_0: u32 = fn_state.accdesc._1;
        // C s_45_1: const #10u : u32
        let s_45_1: u32 = 10;
        // D s_45_2: cmp-eq s_45_0 s_45_1
        let s_45_2: bool = ((s_45_0) == (s_45_1));
        // D s_45_3: write-var gs#20001 <= s_45_2
        fn_state.gs_20001 = s_45_2;
        // N s_45_4: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_46_0: read-var gs#20001:u8
        let s_46_0: bool = fn_state.gs_20001;
        // D s_46_1: not s_46_0
        let s_46_1: bool = !s_46_0;
        // D s_46_2: write-var gs#20002 <= s_46_1
        fn_state.gs_20002 = s_46_1;
        // N s_46_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // D s_47_1: write-var gs#20001 <= s_47_0
        fn_state.gs_20001 = s_47_0;
        // N s_47_2: jump b46
        return block_46(state, tracer, fn_state);
    }
}
