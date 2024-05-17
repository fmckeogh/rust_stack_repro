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
use IsDataAccess::*;
use DBGDSCRext_read::*;
use NoFault__1::*;
use ConstrainUnpredictableBool::*;
use AArch32_CheckBreakpoint::*;
use AArch32_CheckVectorCatch::*;
use u_get_DBGDSCRext_Type_MDBGen::*;
use HaltOnBreakpointOrWatchpoint::*;
use AArch32_GenerateDebugExceptions::*;
use AArch32_CheckWatchpoint::*;
use common::*;
pub fn AArch32_CheckDebug<T: Tracer>(
    state: &mut State,
    tracer: &T,
    vaddress: u32,
    accdesc: ProductType9878976b5bcce9c9,
    size: i128,
) -> ProductType1d757adad216cdef {
    #[derive(Default)]
    struct FunctionState {
        gs_30228: bool,
        i_side: bool,
        gs_30233: bool,
        gs_30234: bool,
        generate_exception: bool,
        gs_30229: bool,
        gs_30227: bool,
        fault: ProductType1d757adad216cdef,
        return_value: ProductType1d757adad216cdef,
        gs_30230: bool,
        d_side: bool,
        gs_30231: bool,
        vector_catch_first_name: bool,
        gs_30226: bool,
        gs_30232: bool,
        halt: bool,
        vaddress: u32,
        accdesc: ProductType9878976b5bcce9c9,
        size: i128,
    }
    let fn_state = FunctionState {
        vaddress,
        accdesc,
        size,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_0_0: read-var accdesc:struct
        let s_0_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_0_1: call NoFault__1(s_0_0)
        let s_0_1: ProductType1d757adad216cdef = NoFault__1(state, tracer, s_0_0);
        // D s_0_2: write-var fault <= s_0_1
        fn_state.fault = s_0_1;
        // D s_0_3: read-var accdesc.1:struct
        let s_0_3: u32 = fn_state.accdesc._1;
        // D s_0_4: call IsDataAccess(s_0_3)
        let s_0_4: bool = IsDataAccess(state, tracer, s_0_3);
        // N s_0_5: branch s_0_4 b42 b1
        if s_0_4 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_1_0: read-var accdesc.1:struct
        let s_1_0: u32 = fn_state.accdesc._1;
        // C s_1_1: const #6u : u32
        let s_1_1: u32 = 6;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // D s_1_3: write-var gs#30226 <= s_1_2
        fn_state.gs_30226 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_2_0: read-var gs#30226:u8
        let s_2_0: bool = fn_state.gs_30226;
        // D s_2_1: write-var d_side <= s_2_0
        fn_state.d_side = s_2_0;
        // D s_2_2: read-var accdesc.1:struct
        let s_2_2: u32 = fn_state.accdesc._1;
        // C s_2_3: const #0u : u32
        let s_2_3: u32 = 0;
        // D s_2_4: cmp-eq s_2_2 s_2_3
        let s_2_4: bool = ((s_2_2) == (s_2_3));
        // D s_2_5: write-var i_side <= s_2_4
        fn_state.i_side = s_2_4;
        // C s_2_6: const #() : ()
        let s_2_6: () = ();
        // S s_2_7: call AArch32_GenerateDebugExceptions(s_2_6)
        let s_2_7: bool = AArch32_GenerateDebugExceptions(state, tracer, s_2_6);
        // N s_2_8: branch s_2_7 b41 b3
        if s_2_7 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#30227 <= s_3_0
        fn_state.gs_30227 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_4_0: read-var gs#30227:u8
        let s_4_0: bool = fn_state.gs_30227;
        // D s_4_1: write-var generate_exception <= s_4_0
        fn_state.generate_exception = s_4_0;
        // C s_4_2: const #() : ()
        let s_4_2: () = ();
        // S s_4_3: call HaltOnBreakpointOrWatchpoint(s_4_2)
        let s_4_3: bool = HaltOnBreakpointOrWatchpoint(state, tracer, s_4_2);
        // D s_4_4: write-var halt <= s_4_3
        fn_state.halt = s_4_3;
        // C s_4_5: const #24u : u32
        let s_4_5: u32 = 24;
        // S s_4_6: call ConstrainUnpredictableBool(s_4_5)
        let s_4_6: bool = ConstrainUnpredictableBool(state, tracer, s_4_5);
        // D s_4_7: write-var vector_catch_first_name <= s_4_6
        fn_state.vector_catch_first_name = s_4_6;
        // D s_4_8: read-var i_side:u8
        let s_4_8: bool = fn_state.i_side;
        // N s_4_9: branch s_4_8 b40 b5
        if s_4_8 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#30228 <= s_5_0
        fn_state.gs_30228 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_6_0: read-var gs#30228:u8
        let s_6_0: bool = fn_state.gs_30228;
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
    ) -> ProductType1d757adad216cdef {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#30229 <= s_7_0
        fn_state.gs_30229 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_8_0: read-var gs#30229:u8
        let s_8_0: bool = fn_state.gs_30229;
        // N s_8_1: branch s_8_0 b38 b9
        if s_8_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_10_0: read-var fault.16:struct
        let s_10_0: u32 = fn_state.fault._16;
        // C s_10_1: const #0u : u32
        let s_10_1: u32 = 0;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // N s_10_3: branch s_10_2 b34 b11
        if s_10_2 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#30231 <= s_11_0
        fn_state.gs_30231 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_12_0: read-var gs#30231:u8
        let s_12_0: bool = fn_state.gs_30231;
        // N s_12_1: branch s_12_0 b27 b13
        if s_12_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_14_0: read-var fault.16:struct
        let s_14_0: u32 = fn_state.fault._16;
        // C s_14_1: const #0u : u32
        let s_14_1: u32 = 0;
        // D s_14_2: cmp-eq s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) == (s_14_1));
        // N s_14_3: branch s_14_2 b26 b15
        if s_14_2 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#30232 <= s_15_0
        fn_state.gs_30232 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_16_0: read-var gs#30232:u8
        let s_16_0: bool = fn_state.gs_30232;
        // N s_16_1: branch s_16_0 b25 b17
        if s_16_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#30233 <= s_17_0
        fn_state.gs_30233 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_18_0: read-var gs#30233:u8
        let s_18_0: bool = fn_state.gs_30233;
        // N s_18_1: branch s_18_0 b24 b19
        if s_18_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#30234 <= s_19_0
        fn_state.gs_30234 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_20_0: read-var gs#30234:u8
        let s_20_0: bool = fn_state.gs_30234;
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
    ) -> ProductType1d757adad216cdef {
        // D s_21_0: read-var fault:struct
        let s_21_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_21_1: write-var return_value <= s_21_0
        fn_state.return_value = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_22_0: read-var return_value:struct
        let s_22_0: ProductType1d757adad216cdef = fn_state.return_value;
        // N s_22_1: return s_22_0
        return s_22_0;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_23_0: read-var fault:struct
        let s_23_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_23_1: read-var vaddress:u32
        let s_23_1: u32 = fn_state.vaddress;
        // D s_23_2: read-var size:i
        let s_23_2: i128 = fn_state.size;
        // D s_23_3: call AArch32_CheckVectorCatch(s_23_0, s_23_1, s_23_2)
        let s_23_3: ProductType1d757adad216cdef = AArch32_CheckVectorCatch(
            state,
            tracer,
            s_23_0,
            s_23_1,
            s_23_2,
        );
        // D s_23_4: write-var return_value <= s_23_3
        fn_state.return_value = s_23_3;
        // N s_23_5: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_24_0: read-var generate_exception:u8
        let s_24_0: bool = fn_state.generate_exception;
        // D s_24_1: write-var gs#30234 <= s_24_0
        fn_state.gs_30234 = s_24_0;
        // N s_24_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_25_0: read-var vector_catch_first_name:u8
        let s_25_0: bool = fn_state.vector_catch_first_name;
        // D s_25_1: not s_25_0
        let s_25_1: bool = !s_25_0;
        // D s_25_2: write-var gs#30233 <= s_25_1
        fn_state.gs_30233 = s_25_1;
        // N s_25_3: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_26_0: read-var i_side:u8
        let s_26_0: bool = fn_state.i_side;
        // D s_26_1: write-var gs#30232 <= s_26_0
        fn_state.gs_30232 = s_26_0;
        // N s_26_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_27_0: read-var d_side:u8
        let s_27_0: bool = fn_state.d_side;
        // N s_27_1: branch s_27_0 b32 b28
        if s_27_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_28_0: read-var i_side:u8
        let s_28_0: bool = fn_state.i_side;
        // N s_28_1: branch s_28_0 b30 b29
        if s_28_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_29_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_30_0: read-var fault:struct
        let s_30_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_30_1: read-var vaddress:u32
        let s_30_1: u32 = fn_state.vaddress;
        // D s_30_2: read-var accdesc:struct
        let s_30_2: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_30_3: read-var size:i
        let s_30_3: i128 = fn_state.size;
        // D s_30_4: call AArch32_CheckBreakpoint(s_30_0, s_30_1, s_30_2, s_30_3)
        let s_30_4: ProductType1d757adad216cdef = AArch32_CheckBreakpoint(
            state,
            tracer,
            s_30_0,
            s_30_1,
            s_30_2,
            s_30_3,
        );
        // D s_30_5: write-var fault <= s_30_4
        fn_state.fault = s_30_4;
        // N s_30_6: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_31_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_32_0: read-var fault:struct
        let s_32_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_32_1: read-var vaddress:u32
        let s_32_1: u32 = fn_state.vaddress;
        // D s_32_2: read-var accdesc:struct
        let s_32_2: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_32_3: read-var size:i
        let s_32_3: i128 = fn_state.size;
        // D s_32_4: call AArch32_CheckWatchpoint(s_32_0, s_32_1, s_32_2, s_32_3)
        let s_32_4: ProductType1d757adad216cdef = AArch32_CheckWatchpoint(
            state,
            tracer,
            s_32_0,
            s_32_1,
            s_32_2,
            s_32_3,
        );
        // D s_32_5: write-var fault <= s_32_4
        fn_state.fault = s_32_4;
        // N s_32_6: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_33_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_34_0: read-var generate_exception:u8
        let s_34_0: bool = fn_state.generate_exception;
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
    ) -> ProductType1d757adad216cdef {
        // D s_35_0: read-var halt:u8
        let s_35_0: bool = fn_state.halt;
        // D s_35_1: write-var gs#30230 <= s_35_0
        fn_state.gs_30230 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_36_0: read-var gs#30230:u8
        let s_36_0: bool = fn_state.gs_30230;
        // D s_36_1: write-var gs#30231 <= s_36_0
        fn_state.gs_30231 = s_36_0;
        // N s_36_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_37_0: const #1u : u8
        let s_37_0: bool = true;
        // D s_37_1: write-var gs#30230 <= s_37_0
        fn_state.gs_30230 = s_37_0;
        // N s_37_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_38_0: read-var fault:struct
        let s_38_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_38_1: read-var vaddress:u32
        let s_38_1: u32 = fn_state.vaddress;
        // D s_38_2: read-var size:i
        let s_38_2: i128 = fn_state.size;
        // D s_38_3: call AArch32_CheckVectorCatch(s_38_0, s_38_1, s_38_2)
        let s_38_3: ProductType1d757adad216cdef = AArch32_CheckVectorCatch(
            state,
            tracer,
            s_38_0,
            s_38_1,
            s_38_2,
        );
        // D s_38_4: write-var fault <= s_38_3
        fn_state.fault = s_38_3;
        // N s_38_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_39_0: read-var generate_exception:u8
        let s_39_0: bool = fn_state.generate_exception;
        // D s_39_1: write-var gs#30229 <= s_39_0
        fn_state.gs_30229 = s_39_0;
        // N s_39_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_40_0: read-var vector_catch_first_name:u8
        let s_40_0: bool = fn_state.vector_catch_first_name;
        // D s_40_1: write-var gs#30228 <= s_40_0
        fn_state.gs_30228 = s_40_0;
        // N s_40_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_41_0: const #() : ()
        let s_41_0: () = ();
        // S s_41_1: call DBGDSCRext_read(s_41_0)
        let s_41_1: ProductType700c18a878c5601b = DBGDSCRext_read(state, tracer, s_41_0);
        // S s_41_2: call _get_DBGDSCRext_Type_MDBGen(s_41_1)
        let s_41_2: bool = u_get_DBGDSCRext_Type_MDBGen(state, tracer, s_41_1);
        // S s_41_3: cast zx s_41_2 -> bv
        let s_41_3: Bits = Bits::new(s_41_2 as u128, 1u16);
        // C s_41_4: const #1u : u8
        let s_41_4: bool = true;
        // C s_41_5: cast zx s_41_4 -> bv
        let s_41_5: Bits = Bits::new(s_41_4 as u128, 1u16);
        // S s_41_6: cmp-eq s_41_3 s_41_5
        let s_41_6: bool = ((s_41_3) == (s_41_5));
        // D s_41_7: write-var gs#30227 <= s_41_6
        fn_state.gs_30227 = s_41_6;
        // N s_41_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // D s_42_1: write-var gs#30226 <= s_42_0
        fn_state.gs_30226 = s_42_0;
        // N s_42_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
