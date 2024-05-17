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
use AArch64_CheckBreakpoint::*;
use NoFault__1::*;
use AArch64_CheckWatchpoint::*;
use AArch64_GenerateDebugExceptions::*;
use CurrentSecurityState::*;
use HaltOnBreakpointOrWatchpoint::*;
use u_get_MDSCR_EL1_Type_MDE::*;
use AArch64_GenerateDebugExceptionsFrom::*;
use common::*;
pub fn AArch64_CheckDebug<T: Tracer>(
    state: &mut State,
    tracer: &T,
    vaddress: u64,
    accdesc: ProductType9878976b5bcce9c9,
    size: i128,
) -> ProductType1d757adad216cdef {
    #[derive(Default)]
    struct FunctionState {
        halt: bool,
        i_side: bool,
        generate_exception: bool,
        fault: ProductType1d757adad216cdef,
        gs_16728: bool,
        d_side: bool,
        gs_16725: bool,
        gs_16729: bool,
        gs_16727: bool,
        vaddress: u64,
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
        // N s_0_5: branch s_0_4 b24 b1
        if s_0_4 {
            return block_24(state, tracer, fn_state);
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
        // D s_1_3: write-var gs#16725 <= s_1_2
        fn_state.gs_16725 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_2_0: read-var gs#16725:u8
        let s_2_0: bool = fn_state.gs_16725;
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
        // D s_2_6: read-var accdesc.1:struct
        let s_2_6: u32 = fn_state.accdesc._1;
        // C s_2_7: const #9u : u32
        let s_2_7: u32 = 9;
        // D s_2_8: cmp-eq s_2_6 s_2_7
        let s_2_8: bool = ((s_2_6) == (s_2_7));
        // N s_2_9: branch s_2_8 b20 b3
        if s_2_8 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call AArch64_GenerateDebugExceptions(s_3_0)
        let s_3_1: bool = AArch64_GenerateDebugExceptions(state, tracer, s_3_0);
        // N s_3_2: branch s_3_1 b19 b4
        if s_3_1 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#16727 <= s_4_0
        fn_state.gs_16727 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_5_0: read-var gs#16727:u8
        let s_5_0: bool = fn_state.gs_16727;
        // D s_5_1: write-var generate_exception <= s_5_0
        fn_state.generate_exception = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call HaltOnBreakpointOrWatchpoint(s_6_0)
        let s_6_1: bool = HaltOnBreakpointOrWatchpoint(state, tracer, s_6_0);
        // D s_6_2: write-var halt <= s_6_1
        fn_state.halt = s_6_1;
        // D s_6_3: read-var generate_exception:u8
        let s_6_3: bool = fn_state.generate_exception;
        // N s_6_4: branch s_6_3 b18 b7
        if s_6_3 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_7_0: read-var halt:u8
        let s_7_0: bool = fn_state.halt;
        // D s_7_1: write-var gs#16729 <= s_7_0
        fn_state.gs_16729 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_8_0: read-var gs#16729:u8
        let s_8_0: bool = fn_state.gs_16729;
        // N s_8_1: branch s_8_0 b11 b9
        if s_8_0 {
            return block_11(state, tracer, fn_state);
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
        // D s_10_0: read-var fault:struct
        let s_10_0: ProductType1d757adad216cdef = fn_state.fault;
        // N s_10_1: return s_10_0
        return s_10_0;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_11_0: read-var d_side:u8
        let s_11_0: bool = fn_state.d_side;
        // N s_11_1: branch s_11_0 b16 b12
        if s_11_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_12_0: read-var i_side:u8
        let s_12_0: bool = fn_state.i_side;
        // N s_12_1: branch s_12_0 b14 b13
        if s_12_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_13_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_14_0: read-var fault:struct
        let s_14_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_14_1: read-var vaddress:u64
        let s_14_1: u64 = fn_state.vaddress;
        // D s_14_2: read-var accdesc:struct
        let s_14_2: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_14_3: read-var size:i
        let s_14_3: i128 = fn_state.size;
        // D s_14_4: call AArch64_CheckBreakpoint(s_14_0, s_14_1, s_14_2, s_14_3)
        let s_14_4: ProductType1d757adad216cdef = AArch64_CheckBreakpoint(
            state,
            tracer,
            s_14_0,
            s_14_1,
            s_14_2,
            s_14_3,
        );
        // D s_14_5: write-var fault <= s_14_4
        fn_state.fault = s_14_4;
        // N s_14_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_15_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_16_0: read-var fault:struct
        let s_16_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_16_1: read-var vaddress:u64
        let s_16_1: u64 = fn_state.vaddress;
        // D s_16_2: read-var accdesc:struct
        let s_16_2: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_16_3: read-var size:i
        let s_16_3: i128 = fn_state.size;
        // D s_16_4: call AArch64_CheckWatchpoint(s_16_0, s_16_1, s_16_2, s_16_3)
        let s_16_4: ProductType1d757adad216cdef = AArch64_CheckWatchpoint(
            state,
            tracer,
            s_16_0,
            s_16_1,
            s_16_2,
            s_16_3,
        );
        // D s_16_5: write-var fault <= s_16_4
        fn_state.fault = s_16_4;
        // N s_16_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_17_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#16729 <= s_18_0
        fn_state.gs_16729 = s_18_0;
        // N s_18_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_19_0: const #104648u : u32
        let s_19_0: u32 = 104648;
        // D s_19_1: read-reg s_19_0:struct
        let s_19_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call _get_MDSCR_EL1_Type_MDE(s_19_1)
        let s_19_2: bool = u_get_MDSCR_EL1_Type_MDE(state, tracer, s_19_1);
        // D s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // C s_19_4: const #1u : u8
        let s_19_4: bool = true;
        // C s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 1u16);
        // D s_19_6: cmp-eq s_19_3 s_19_5
        let s_19_6: bool = ((s_19_3) == (s_19_5));
        // D s_19_7: write-var gs#16727 <= s_19_6
        fn_state.gs_16727 = s_19_6;
        // N s_19_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // C s_20_1: const #() : ()
        let s_20_1: () = ();
        // S s_20_2: call CurrentSecurityState(s_20_1)
        let s_20_2: u32 = CurrentSecurityState(state, tracer, s_20_1);
        // C s_20_3: const #432u : u32
        let s_20_3: u32 = 432;
        // D s_20_4: read-reg s_20_3:u8
        let s_20_4: u8 = {
            let value = state.read_register::<u8>(s_20_3 as isize);
            tracer.read_register(s_20_3 as isize, value);
            value
        };
        // D s_20_5: call AArch64_GenerateDebugExceptionsFrom(s_20_4, s_20_2, s_20_0)
        let s_20_5: bool = AArch64_GenerateDebugExceptionsFrom(
            state,
            tracer,
            s_20_4,
            s_20_2,
            s_20_0,
        );
        // N s_20_6: branch s_20_5 b23 b21
        if s_20_5 {
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
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#16728 <= s_21_0
        fn_state.gs_16728 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_22_0: read-var gs#16728:u8
        let s_22_0: bool = fn_state.gs_16728;
        // D s_22_1: write-var generate_exception <= s_22_0
        fn_state.generate_exception = s_22_0;
        // N s_22_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_23_0: const #104648u : u32
        let s_23_0: u32 = 104648;
        // D s_23_1: read-reg s_23_0:struct
        let s_23_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call _get_MDSCR_EL1_Type_MDE(s_23_1)
        let s_23_2: bool = u_get_MDSCR_EL1_Type_MDE(state, tracer, s_23_1);
        // D s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // C s_23_4: const #1u : u8
        let s_23_4: bool = true;
        // C s_23_5: cast zx s_23_4 -> bv
        let s_23_5: Bits = Bits::new(s_23_4 as u128, 1u16);
        // D s_23_6: cmp-eq s_23_3 s_23_5
        let s_23_6: bool = ((s_23_3) == (s_23_5));
        // D s_23_7: write-var gs#16728 <= s_23_6
        fn_state.gs_16728 = s_23_6;
        // N s_23_8: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#16725 <= s_24_0
        fn_state.gs_16725 = s_24_0;
        // N s_24_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
