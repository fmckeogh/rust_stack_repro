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
use AArch64_WatchpointMatch::*;
use NumWatchpointsImplemented::*;
use u__id::*;
use u_get_DBGWCR_EL1_Type_LSC::*;
use ELUsingAArch32::*;
use Halt::*;
use S1TranslationRegime__1::*;
use HaltOnBreakpointOrWatchpoint::*;
use common::*;
pub fn AArch64_CheckWatchpoint<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault_in: ProductType1d757adad216cdef,
    vaddress: u64,
    accdesc: ProductType9878976b5bcce9c9,
    size: i128,
) -> ProductType1d757adad216cdef {
    #[derive(Default)]
    struct FunctionState {
        gs_16710: bool,
        return_value: ProductType1d757adad216cdef,
        fault: ProductType1d757adad216cdef,
        gs_16717: bool,
        gs_16718: bool,
        gs_16716: bool,
        i: i64,
        gs_16702: i64,
        gs_16719: bool,
        gs_16713: bool,
        fault_in: ProductType1d757adad216cdef,
        vaddress: u64,
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
        // S s_0_3: not s_0_2
        let s_0_3: bool = !s_0_2;
        // N s_0_4: assert s_0_3
        let s_0_4: () = assert!(s_0_3);
        // D s_0_5: read-var fault_in:struct
        let s_0_5: ProductType1d757adad216cdef = fn_state.fault_in;
        // D s_0_6: write-var fault <= s_0_5
        fn_state.fault = s_0_5;
        // D s_0_7: read-var accdesc.1:struct
        let s_0_7: u32 = fn_state.accdesc._1;
        // C s_0_8: const #6u : u32
        let s_0_8: u32 = 6;
        // D s_0_9: cmp-eq s_0_7 s_0_8
        let s_0_9: bool = ((s_0_7) == (s_0_8));
        // N s_0_10: branch s_0_9 b37 b1
        if s_0_9 {
            return block_37(state, tracer, fn_state);
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
        // N s_1_3: branch s_1_2 b36 b2
        if s_1_2 {
            return block_36(state, tracer, fn_state);
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
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // C s_3_1: const #() : ()
        let s_3_1: () = ();
        // S s_3_2: call NumWatchpointsImplemented(s_3_1)
        let s_3_2: i128 = NumWatchpointsImplemented(state, tracer, s_3_1);
        // C s_3_3: const #1s : i
        let s_3_3: i128 = 1;
        // S s_3_4: sub s_3_2 s_3_3
        let s_3_4: i128 = ((s_3_2) - (s_3_3));
        // S s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: write-var gs#16702 <= s_3_5
        fn_state.gs_16702 = s_3_5;
        // D s_3_7: write-var i <= s_3_0
        fn_state.i = s_3_0;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_4_0: read-var i:i64
        let s_4_0: i64 = fn_state.i;
        // D s_4_1: read-var gs#16702:i64
        let s_4_1: i64 = fn_state.gs_16702;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b19 b5
        if s_4_2 {
            return block_19(state, tracer, fn_state);
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
        // C s_5_3: const #64s : i
        let s_5_3: i128 = 64;
        // D s_5_4: cmp-lt s_5_2 s_5_3
        let s_5_4: bool = ((s_5_2) < (s_5_3));
        // N s_5_5: assert s_5_4
        let s_5_5: () = assert!(s_5_4);
        // D s_5_6: read-var i:i64
        let s_5_6: i64 = fn_state.i;
        // D s_5_7: read-var vaddress:u64
        let s_5_7: u64 = fn_state.vaddress;
        // D s_5_8: read-var size:i
        let s_5_8: i128 = fn_state.size;
        // D s_5_9: read-var accdesc:struct
        let s_5_9: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_5_10: call AArch64_WatchpointMatch(s_5_6, s_5_7, s_5_8, s_5_9)
        let s_5_10: bool = AArch64_WatchpointMatch(
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
        // C s_8_0: const #17u : u32
        let s_8_0: u32 = 17;
        // D s_8_1: write-var fault.16 <= s_8_0
        fn_state.fault._16 = s_8_0;
        // C s_8_2: const #103984u : u32
        let s_8_2: u32 = 103984;
        // D s_8_3: read-reg s_8_2:[struct; 64]
        let s_8_3: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 64usize]>(s_8_2 as isize);
            tracer.read_register(s_8_2 as isize, value);
            value
        };
        // D s_8_4: read-var i:i64
        let s_8_4: i64 = fn_state.i;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: read-element s_8_3[s_8_5]
        let s_8_6: ProductType5c790c8ef59cc8b2 = s_8_3[(s_8_5) as usize];
        // D s_8_7: call _get_DBGWCR_EL1_Type_LSC(s_8_6)
        let s_8_7: u8 = u_get_DBGWCR_EL1_Type_LSC(state, tracer, s_8_6);
        // C s_8_8: const #0s : i
        let s_8_8: i128 = 0;
        // D s_8_9: cast zx s_8_7 -> bv
        let s_8_9: Bits = Bits::new(s_8_7 as u128, 2u16);
        // C s_8_10: const #1u : u64
        let s_8_10: u64 = 1;
        // D s_8_11: bit-extract s_8_9 s_8_8 s_8_10
        let s_8_11: Bits = (Bits::new(
            ((s_8_9) >> (s_8_8)).value(),
            u16::try_from(s_8_10).unwrap(),
        ));
        // D s_8_12: cast reint s_8_11 -> u8
        let s_8_12: bool = ((s_8_11.value()) != 0);
        // C s_8_13: const #0s : i
        let s_8_13: i128 = 0;
        // C s_8_14: const #0u : u64
        let s_8_14: u64 = 0;
        // D s_8_15: cast zx s_8_12 -> u64
        let s_8_15: u64 = (s_8_12 as u64);
        // C s_8_16: const #1u : u64
        let s_8_16: u64 = 1;
        // D s_8_17: and s_8_15 s_8_16
        let s_8_17: u64 = ((s_8_15) & (s_8_16));
        // D s_8_18: cmp-eq s_8_17 s_8_16
        let s_8_18: bool = ((s_8_17) == (s_8_16));
        // D s_8_19: lsl s_8_15 s_8_13
        let s_8_19: u64 = s_8_15 << s_8_13;
        // D s_8_20: or s_8_14 s_8_19
        let s_8_20: u64 = ((s_8_14) | (s_8_19));
        // D s_8_21: cmpl s_8_19
        let s_8_21: u64 = !s_8_19;
        // D s_8_22: and s_8_14 s_8_21
        let s_8_22: u64 = ((s_8_14) & (s_8_21));
        // D s_8_23: select s_8_18 s_8_20 s_8_22
        let s_8_23: u64 = if s_8_18 { s_8_20 } else { s_8_22 };
        // D s_8_24: cast trunc s_8_23 -> u8
        let s_8_24: bool = ((s_8_23) != 0);
        // D s_8_25: cast zx s_8_24 -> bv
        let s_8_25: Bits = Bits::new(s_8_24 as u128, 1u16);
        // C s_8_26: const #1u : u8
        let s_8_26: bool = true;
        // C s_8_27: cast zx s_8_26 -> bv
        let s_8_27: Bits = Bits::new(s_8_26 as u128, 1u16);
        // D s_8_28: cmp-eq s_8_25 s_8_27
        let s_8_28: bool = ((s_8_25) == (s_8_27));
        // N s_8_29: branch s_8_28 b18 b9
        if s_8_28 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#16710 <= s_9_0
        fn_state.gs_16710 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_10_0: read-var gs#16710:u8
        let s_10_0: bool = fn_state.gs_16710;
        // N s_10_1: branch s_10_0 b17 b11
        if s_10_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_11_0: const #103984u : u32
        let s_11_0: u32 = 103984;
        // D s_11_1: read-reg s_11_0:[struct; 64]
        let s_11_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: read-var i:i64
        let s_11_2: i64 = fn_state.i;
        // D s_11_3: cast zx s_11_2 -> i
        let s_11_3: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_4: read-element s_11_1[s_11_3]
        let s_11_4: ProductType5c790c8ef59cc8b2 = s_11_1[(s_11_3) as usize];
        // D s_11_5: call _get_DBGWCR_EL1_Type_LSC(s_11_4)
        let s_11_5: u8 = u_get_DBGWCR_EL1_Type_LSC(state, tracer, s_11_4);
        // C s_11_6: const #1s : i
        let s_11_6: i128 = 1;
        // D s_11_7: cast zx s_11_5 -> bv
        let s_11_7: Bits = Bits::new(s_11_5 as u128, 2u16);
        // C s_11_8: const #1u : u64
        let s_11_8: u64 = 1;
        // D s_11_9: bit-extract s_11_7 s_11_6 s_11_8
        let s_11_9: Bits = (Bits::new(
            ((s_11_7) >> (s_11_6)).value(),
            u16::try_from(s_11_8).unwrap(),
        ));
        // D s_11_10: cast reint s_11_9 -> u8
        let s_11_10: bool = ((s_11_9.value()) != 0);
        // C s_11_11: const #0s : i
        let s_11_11: i128 = 0;
        // C s_11_12: const #0u : u64
        let s_11_12: u64 = 0;
        // D s_11_13: cast zx s_11_10 -> u64
        let s_11_13: u64 = (s_11_10 as u64);
        // C s_11_14: const #1u : u64
        let s_11_14: u64 = 1;
        // D s_11_15: and s_11_13 s_11_14
        let s_11_15: u64 = ((s_11_13) & (s_11_14));
        // D s_11_16: cmp-eq s_11_15 s_11_14
        let s_11_16: bool = ((s_11_15) == (s_11_14));
        // D s_11_17: lsl s_11_13 s_11_11
        let s_11_17: u64 = s_11_13 << s_11_11;
        // D s_11_18: or s_11_12 s_11_17
        let s_11_18: u64 = ((s_11_12) | (s_11_17));
        // D s_11_19: cmpl s_11_17
        let s_11_19: u64 = !s_11_17;
        // D s_11_20: and s_11_12 s_11_19
        let s_11_20: u64 = ((s_11_12) & (s_11_19));
        // D s_11_21: select s_11_16 s_11_18 s_11_20
        let s_11_21: u64 = if s_11_16 { s_11_18 } else { s_11_20 };
        // D s_11_22: cast trunc s_11_21 -> u8
        let s_11_22: bool = ((s_11_21) != 0);
        // D s_11_23: cast zx s_11_22 -> bv
        let s_11_23: Bits = Bits::new(s_11_22 as u128, 1u16);
        // C s_11_24: const #1u : u8
        let s_11_24: bool = true;
        // C s_11_25: cast zx s_11_24 -> bv
        let s_11_25: Bits = Bits::new(s_11_24 as u128, 1u16);
        // D s_11_26: cmp-eq s_11_23 s_11_25
        let s_11_26: bool = ((s_11_23) == (s_11_25));
        // N s_11_27: branch s_11_26 b16 b12
        if s_11_26 {
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
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#16713 <= s_12_0
        fn_state.gs_16713 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_13_0: read-var gs#16713:u8
        let s_13_0: bool = fn_state.gs_16713;
        // N s_13_1: branch s_13_0 b15 b14
        if s_13_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_14_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var fault.19 <= s_15_0
        fn_state.fault._19 = s_15_0;
        // N s_15_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_16_0: read-var accdesc.32:struct
        let s_16_0: bool = fn_state.accdesc._32;
        // D s_16_1: write-var gs#16713 <= s_16_0
        fn_state.gs_16713 = s_16_0;
        // N s_16_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var fault.19 <= s_17_0
        fn_state.fault._19 = s_17_0;
        // N s_17_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_18_0: read-var accdesc.23:struct
        let s_18_0: bool = fn_state.accdesc._23;
        // D s_18_1: write-var gs#16710 <= s_18_0
        fn_state.gs_16710 = s_18_0;
        // N s_18_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_19_0: read-var fault.16:struct
        let s_19_0: u32 = fn_state.fault._16;
        // C s_19_1: const #17u : u32
        let s_19_1: u32 = 17;
        // D s_19_2: cmp-eq s_19_0 s_19_1
        let s_19_2: bool = ((s_19_0) == (s_19_1));
        // N s_19_3: branch s_19_2 b35 b20
        if s_19_2 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#16716 <= s_20_0
        fn_state.gs_16716 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_21_0: read-var gs#16716:u8
        let s_21_0: bool = fn_state.gs_16716;
        // N s_21_1: branch s_21_0 b34 b22
        if s_21_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#16717 <= s_22_0
        fn_state.gs_16717 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_23_0: read-var gs#16717:u8
        let s_23_0: bool = fn_state.gs_16717;
        // N s_23_1: branch s_23_0 b30 b24
        if s_23_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var gs#16719 <= s_24_0
        fn_state.gs_16719 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_25_0: read-var gs#16719:u8
        let s_25_0: bool = fn_state.gs_16719;
        // N s_25_1: branch s_25_0 b29 b26
        if s_25_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_26_0: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_27_0: read-var fault:struct
        let s_27_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_27_1: write-var return_value <= s_27_0
        fn_state.return_value = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_28_0: read-var return_value:struct
        let s_28_0: ProductType1d757adad216cdef = fn_state.return_value;
        // N s_28_1: return s_28_0
        return s_28_0;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_29_0: const #1144u : u32
        let s_29_0: u32 = 1144;
        // D s_29_1: read-reg s_29_0:u8
        let s_29_1: u8 = {
            let value = state.read_register::<u8>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: read-var vaddress:u64
        let s_29_2: u64 = fn_state.vaddress;
        // C s_29_3: const #16472u : u32
        let s_29_3: u32 = 16472;
        // N s_29_4: write-reg s_29_3 <= s_29_2
        let s_29_4: () = {
            state.write_register::<u64>(s_29_3 as isize, s_29_2);
            tracer.write_register(s_29_3 as isize, s_29_2);
        };
        // D s_29_5: call Halt(s_29_1)
        let s_29_5: () = Halt(state, tracer, s_29_1);
        // N s_29_6: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_30_0: read-var accdesc.11:struct
        let s_30_0: bool = fn_state.accdesc._11;
        // N s_30_1: branch s_30_0 b33 b31
        if s_30_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var gs#16718 <= s_31_0
        fn_state.gs_16718 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_32_0: read-var gs#16718:u8
        let s_32_0: bool = fn_state.gs_16718;
        // D s_32_1: not s_32_0
        let s_32_1: bool = !s_32_0;
        // D s_32_2: write-var gs#16719 <= s_32_1
        fn_state.gs_16719 = s_32_1;
        // N s_32_3: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_33_0: read-var accdesc.10:struct
        let s_33_0: bool = fn_state.accdesc._10;
        // D s_33_1: not s_33_0
        let s_33_1: bool = !s_33_0;
        // D s_33_2: write-var gs#16718 <= s_33_1
        fn_state.gs_16718 = s_33_1;
        // N s_33_3: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_34_0: read-var accdesc.17:struct
        let s_34_0: bool = fn_state.accdesc._17;
        // D s_34_1: not s_34_0
        let s_34_1: bool = !s_34_0;
        // D s_34_2: write-var gs#16717 <= s_34_1
        fn_state.gs_16717 = s_34_1;
        // N s_34_3: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_35_0: const #() : ()
        let s_35_0: () = ();
        // S s_35_1: call HaltOnBreakpointOrWatchpoint(s_35_0)
        let s_35_1: bool = HaltOnBreakpointOrWatchpoint(state, tracer, s_35_0);
        // D s_35_2: write-var gs#16716 <= s_35_1
        fn_state.gs_16716 = s_35_1;
        // N s_35_3: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_36_0: read-var fault:struct
        let s_36_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_36_1: write-var return_value <= s_36_0
        fn_state.return_value = s_36_0;
        // N s_36_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_37_0: read-var accdesc.5:struct
        let s_37_0: u32 = fn_state.accdesc._5;
        // C s_37_1: const #1u : u32
        let s_37_1: u32 = 1;
        // D s_37_2: cmp-eq s_37_0 s_37_1
        let s_37_2: bool = ((s_37_0) == (s_37_1));
        // N s_37_3: branch s_37_2 b39 b38
        if s_37_2 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_38_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_39_0: read-var fault:struct
        let s_39_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_39_1: write-var return_value <= s_39_0
        fn_state.return_value = s_39_0;
        // N s_39_2: jump b28
        return block_28(state, tracer, fn_state);
    }
}
