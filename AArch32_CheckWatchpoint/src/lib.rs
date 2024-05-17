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
use AArch32_WatchpointMatch::*;
use NumWatchpointsImplemented::*;
use u__id::*;
use ELUsingAArch32::*;
use Halt::*;
use S1TranslationRegime__1::*;
use u__IMPDEF_boolean::*;
use HaltOnBreakpointOrWatchpoint::*;
use common::*;
pub fn AArch32_CheckWatchpoint<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault_in: ProductType1d757adad216cdef,
    vaddress: u32,
    accdesc: ProductType9878976b5bcce9c9,
    size: i128,
) -> ProductType1d757adad216cdef {
    #[derive(Default)]
    struct FunctionState {
        val_match: bool,
        gs_30213: i64,
        i: i64,
        return_value: ProductType1d757adad216cdef,
        fault: ProductType1d757adad216cdef,
        gs_30220: bool,
        fault_in: ProductType1d757adad216cdef,
        vaddress: u32,
        accdesc: ProductType9878976b5bcce9c9,
        size: i128,
    }
    let fn_state = FunctionState {
        fault_in,
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
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call S1TranslationRegime__1(s_0_0)
        let s_0_1: u8 = S1TranslationRegime__1(state, tracer, s_0_0);
        // S s_0_2: call ELUsingAArch32(s_0_1)
        let s_0_2: bool = ELUsingAArch32(state, tracer, s_0_1);
        // N s_0_3: assert s_0_2
        let s_0_3: () = assert!(s_0_2);
        // D s_0_4: read-var fault_in:struct
        let s_0_4: ProductType1d757adad216cdef = fn_state.fault_in;
        // D s_0_5: write-var fault <= s_0_4
        fn_state.fault = s_0_4;
        // D s_0_6: read-var accdesc.1:struct
        let s_0_6: u32 = fn_state.accdesc._1;
        // C s_0_7: const #6u : u32
        let s_0_7: u32 = 6;
        // D s_0_8: cmp-eq s_0_6 s_0_7
        let s_0_8: bool = ((s_0_6) == (s_0_7));
        // N s_0_9: branch s_0_8 b20 b1
        if s_0_8 {
            return block_20(state, tracer, fn_state);
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
        // D s_1_1: call IsDataAccess(s_1_0)
        let s_1_1: bool = IsDataAccess(state, tracer, s_1_0);
        // D s_1_2: not s_1_1
        let s_1_2: bool = !s_1_1;
        // N s_1_3: branch s_1_2 b19 b2
        if s_1_2 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_2_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var val_match <= s_3_0
        fn_state.val_match = s_3_0;
        // C s_3_2: const #0s : i64
        let s_3_2: i64 = 0;
        // C s_3_3: const #() : ()
        let s_3_3: () = ();
        // S s_3_4: call NumWatchpointsImplemented(s_3_3)
        let s_3_4: i128 = NumWatchpointsImplemented(state, tracer, s_3_3);
        // C s_3_5: const #1s : i
        let s_3_5: i128 = 1;
        // S s_3_6: sub s_3_4 s_3_5
        let s_3_6: i128 = ((s_3_4) - (s_3_5));
        // S s_3_7: cast reint s_3_6 -> i64
        let s_3_7: i64 = (s_3_6 as i64);
        // D s_3_8: write-var gs#30213 <= s_3_7
        fn_state.gs_30213 = s_3_7;
        // D s_3_9: write-var i <= s_3_2
        fn_state.i = s_3_2;
        // N s_3_10: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_4_0: read-var i:i64
        let s_4_0: i64 = fn_state.i;
        // D s_4_1: read-var gs#30213:i64
        let s_4_1: i64 = fn_state.gs_30213;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b9 b5
        if s_4_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_5_0: read-var i:i64
        let s_5_0: i64 = fn_state.i;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: call __id(s_5_1)
        let s_5_2: i128 = u__id(state, tracer, s_5_1);
        // C s_5_3: const #16s : i
        let s_5_3: i128 = 16;
        // D s_5_4: cmp-lt s_5_2 s_5_3
        let s_5_4: bool = ((s_5_2) < (s_5_3));
        // N s_5_5: assert s_5_4
        let s_5_5: () = assert!(s_5_4);
        // D s_5_6: read-var i:i64
        let s_5_6: i64 = fn_state.i;
        // D s_5_7: read-var vaddress:u32
        let s_5_7: u32 = fn_state.vaddress;
        // D s_5_8: read-var size:i
        let s_5_8: i128 = fn_state.size;
        // D s_5_9: read-var accdesc:struct
        let s_5_9: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_5_10: call AArch32_WatchpointMatch(s_5_6, s_5_7, s_5_8, s_5_9)
        let s_5_10: bool = AArch32_WatchpointMatch(
            state,
            tracer,
            s_5_6,
            s_5_7,
            s_5_8,
            s_5_9,
        );
        // N s_5_11: branch s_5_10 b8 b6
        if s_5_10 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_7_0: read-var i:i64
        let s_7_0: i64 = fn_state.i;
        // C s_7_1: const #1s : i64
        let s_7_1: i64 = 1;
        // D s_7_2: add s_7_0 s_7_1
        let s_7_2: i64 = (s_7_0 + s_7_1);
        // D s_7_3: write-var i <= s_7_2
        fn_state.i = s_7_2;
        // N s_7_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // D s_8_1: write-var val_match <= s_8_0
        fn_state.val_match = s_8_0;
        // N s_8_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_9_0: read-var val_match:u8
        let s_9_0: bool = fn_state.val_match;
        // N s_9_1: branch s_9_0 b18 b10
        if s_9_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#30220 <= s_10_0
        fn_state.gs_30220 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_11_0: read-var gs#30220:u8
        let s_11_0: bool = fn_state.gs_30220;
        // N s_11_1: branch s_11_0 b17 b12
        if s_11_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_12_0: read-var val_match:u8
        let s_12_0: bool = fn_state.val_match;
        // N s_12_1: branch s_12_0 b16 b13
        if s_12_0 {
            return block_16(state, tracer, fn_state);
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
        // D s_14_0: read-var fault:struct
        let s_14_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_14_1: write-var return_value <= s_14_0
        fn_state.return_value = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_15_0: read-var return_value:struct
        let s_15_0: ProductType1d757adad216cdef = fn_state.return_value;
        // N s_15_1: return s_15_0
        return s_15_0;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_16_0: const #17u : u32
        let s_16_0: u32 = 17;
        // D s_16_1: write-var fault.16 <= s_16_0
        fn_state.fault._16 = s_16_0;
        // C s_16_2: const #1336u : u32
        let s_16_2: u32 = 1336;
        // D s_16_3: read-reg s_16_2:u8
        let s_16_3: u8 = {
            let value = state.read_register::<u8>(s_16_2 as isize);
            tracer.read_register(s_16_2 as isize, value);
            value
        };
        // D s_16_4: write-var fault.2 <= s_16_3
        fn_state.fault._2 = s_16_3;
        // N s_16_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_17_0: const #1144u : u32
        let s_17_0: u32 = 1144;
        // D s_17_1: read-reg s_17_0:u8
        let s_17_1: u8 = {
            let value = state.read_register::<u8>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // C s_17_2: const #64s : i
        let s_17_2: i128 = 64;
        // D s_17_3: read-var vaddress:u32
        let s_17_3: u32 = fn_state.vaddress;
        // D s_17_4: cast zx s_17_3 -> bv
        let s_17_4: Bits = Bits::new(s_17_3 as u128, 32u16);
        // D s_17_5: bits-cast zx s_17_4 -> bv length s_17_2
        let s_17_5: Bits = s_17_4.zero_extend(s_17_2);
        // D s_17_6: cast reint s_17_5 -> u64
        let s_17_6: u64 = (s_17_5.value() as u64);
        // C s_17_7: const #16472u : u32
        let s_17_7: u32 = 16472;
        // N s_17_8: write-reg s_17_7 <= s_17_6
        let s_17_8: () = {
            state.write_register::<u64>(s_17_7 as isize, s_17_6);
            tracer.write_register(s_17_7 as isize, s_17_6);
        };
        // D s_17_9: call Halt(s_17_1)
        let s_17_9: () = Halt(state, tracer, s_17_1);
        // N s_17_10: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call HaltOnBreakpointOrWatchpoint(s_18_0)
        let s_18_1: bool = HaltOnBreakpointOrWatchpoint(state, tracer, s_18_0);
        // D s_18_2: write-var gs#30220 <= s_18_1
        fn_state.gs_30220 = s_18_1;
        // N s_18_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_19_0: read-var fault:struct
        let s_19_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_19_1: write-var return_value <= s_19_0
        fn_state.return_value = s_19_0;
        // N s_19_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_20_0: read-var accdesc.5:struct
        let s_20_0: u32 = fn_state.accdesc._5;
        // C s_20_1: const #1u : u32
        let s_20_1: u32 = 1;
        // D s_20_2: cmp-eq s_20_0 s_20_1
        let s_20_2: bool = ((s_20_0) == (s_20_1));
        // N s_20_3: branch s_20_2 b24 b21
        if s_20_2 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_21_0: const #"DCIMVAC generates watchpoint" : str
        let s_21_0: &'static str = "DCIMVAC generates watchpoint";
        // S s_21_1: call __IMPDEF_boolean(s_21_0)
        let s_21_1: bool = u__IMPDEF_boolean(state, tracer, s_21_0);
        // S s_21_2: not s_21_1
        let s_21_2: bool = !s_21_1;
        // N s_21_3: branch s_21_2 b23 b22
        if s_21_2 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_22_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_23_0: read-var fault:struct
        let s_23_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_23_1: write-var return_value <= s_23_0
        fn_state.return_value = s_23_0;
        // N s_23_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_24_0: read-var fault:struct
        let s_24_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_24_1: write-var return_value <= s_24_0
        fn_state.return_value = s_24_0;
        // N s_24_2: jump b15
        return block_15(state, tracer, fn_state);
    }
}
