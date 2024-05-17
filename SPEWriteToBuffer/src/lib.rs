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
use ProfilingBufferEnabled::*;
use u_get_PMBPTR_EL1_Type_PTR::*;
use IsFault__2::*;
use DebugWriteFault::*;
use ProfilingBufferOwner::*;
use u__IMPDEF_boolean::*;
use is_zero_subrange::*;
use CreateAccDescSPE::*;
use u__id::*;
use IsFault__1::*;
use IsZero::*;
use u_get_PMBIDR_EL1_Type_Align::*;
use Mk_PMBPTR_EL1_Type::*;
use DebugWriteExternalAbort::*;
use DebugMemWrite::*;
use u_get_PMBSR_EL1_Type_S::*;
use common::*;
pub fn SPEWriteToBuffer<T: Tracer>(state: &mut State, tracer: &T, gs_26020: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_20353: ProductTypec8897aad3eb4a29e,
        ttw_fault: bool,
        fault: ProductType1d757adad216cdef,
        gs_26033: i64,
        gs_26048: bool,
        memstatus: ProductTypef8c3639b88223255,
        ttw_fault_as_external_abort: bool,
        gs_26049: bool,
        accdesc: ProductType9878976b5bcce9c9,
        addrdesc: ProductTypece7c66ccb2cab13e,
        start_vaddr: u64,
        ga_20363: ProductType7b38a52e3b2f4e94,
        i: i64,
        aligned: bool,
        gs_26054: bool,
        gs_26055: bool,
        gs_26039: bool,
        gs_26050: bool,
        gs_26020: (),
    }
    let fn_state = FunctionState {
        gs_26020,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call ProfilingBufferEnabled(s_0_0)
        let s_0_1: bool = ProfilingBufferEnabled(state, tracer, s_0_0);
        // N s_0_2: assert s_0_1
        let s_0_2: () = assert!(s_0_1);
        // C s_0_3: const #90512u : u32
        let s_0_3: u32 = 90512;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_PMBPTR_EL1_Type_PTR(s_0_4)
        let s_0_5: u64 = u_get_PMBPTR_EL1_Type_PTR(state, tracer, s_0_4);
        // C s_0_6: const #15704u : u32
        let s_0_6: u32 = 15704;
        // D s_0_7: read-reg s_0_6:struct
        let s_0_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_6 as isize);
            tracer.read_register(s_0_6 as isize, value);
            value
        };
        // D s_0_8: call _get_PMBIDR_EL1_Type_Align(s_0_7)
        let s_0_8: u8 = u_get_PMBIDR_EL1_Type_Align(state, tracer, s_0_7);
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 4u16);
        // D s_0_10: cast zx s_0_9 -> i
        let s_0_10: i128 = (s_0_9.value() as i128);
        // D s_0_11: cast reint s_0_10 -> i64
        let s_0_11: i64 = (s_0_10 as i64);
        // C s_0_12: const #1s : i
        let s_0_12: i128 = 1;
        // D s_0_13: cast zx s_0_11 -> i
        let s_0_13: i128 = (i128::try_from(s_0_11).unwrap());
        // D s_0_14: sub s_0_13 s_0_12
        let s_0_14: i128 = ((s_0_13) - (s_0_12));
        // D s_0_15: cast reint s_0_14 -> i64
        let s_0_15: i64 = (s_0_14 as i64);
        // C s_0_16: const #0s : i
        let s_0_16: i128 = 0;
        // D s_0_17: cast zx s_0_5 -> bv
        let s_0_17: Bits = Bits::new(s_0_5 as u128, 64u16);
        // D s_0_18: cast zx s_0_15 -> i
        let s_0_18: i128 = (i128::try_from(s_0_15).unwrap());
        // D s_0_19: call is_zero_subrange(s_0_17, s_0_18, s_0_16)
        let s_0_19: bool = is_zero_subrange(state, tracer, s_0_17, s_0_18, s_0_16);
        // D s_0_20: write-var aligned <= s_0_19
        fn_state.aligned = s_0_19;
        // C s_0_21: const #"SPE TTW fault External abort" : str
        let s_0_21: &'static str = "SPE TTW fault External abort";
        // S s_0_22: call __IMPDEF_boolean(s_0_21)
        let s_0_22: bool = u__IMPDEF_boolean(state, tracer, s_0_21);
        // D s_0_23: write-var ttw_fault_as_external_abort <= s_0_22
        fn_state.ttw_fault_as_external_abort = s_0_22;
        // C s_0_24: const #() : ()
        let s_0_24: () = ();
        // S s_0_25: call ProfilingBufferOwner(s_0_24)
        let s_0_25: ProductTypec8897aad3eb4a29e = ProfilingBufferOwner(
            state,
            tracer,
            s_0_24,
        );
        // D s_0_26: write-var ga#20353 <= s_0_25
        fn_state.ga_20353 = s_0_25;
        // D s_0_27: read-var ga#20353.0:struct
        let s_0_27: u32 = fn_state.ga_20353._0;
        // D s_0_28: read-var ga#20353.1:struct
        let s_0_28: u8 = fn_state.ga_20353._1;
        // D s_0_29: call CreateAccDescSPE(s_0_27, s_0_28)
        let s_0_29: ProductType9878976b5bcce9c9 = CreateAccDescSPE(
            state,
            tracer,
            s_0_27,
            s_0_28,
        );
        // D s_0_30: write-var accdesc <= s_0_29
        fn_state.accdesc = s_0_29;
        // C s_0_31: const #90512u : u32
        let s_0_31: u32 = 90512;
        // D s_0_32: read-reg s_0_31:u64
        let s_0_32: u64 = {
            let value = state.read_register::<u64>(s_0_31 as isize);
            tracer.read_register(s_0_31 as isize, value);
            value
        };
        // C s_0_33: const #0s : i
        let s_0_33: i128 = 0;
        // D s_0_34: cast zx s_0_32 -> bv
        let s_0_34: Bits = Bits::new(s_0_32 as u128, 64u16);
        // C s_0_35: const #1s : i64
        let s_0_35: i64 = 1;
        // C s_0_36: cast zx s_0_35 -> i
        let s_0_36: i128 = (i128::try_from(s_0_35).unwrap());
        // C s_0_37: const #63s : i
        let s_0_37: i128 = 63;
        // C s_0_38: add s_0_37 s_0_36
        let s_0_38: i128 = (s_0_37 + s_0_36);
        // D s_0_39: bit-extract s_0_34 s_0_33 s_0_38
        let s_0_39: Bits = (Bits::new(
            ((s_0_34) >> (s_0_33)).value(),
            u16::try_from(s_0_38).unwrap(),
        ));
        // D s_0_40: cast reint s_0_39 -> u64
        let s_0_40: u64 = (s_0_39.value() as u64);
        // D s_0_41: write-var start_vaddr <= s_0_40
        fn_state.start_vaddr = s_0_40;
        // C s_0_42: const #0s : i64
        let s_0_42: i64 = 0;
        // C s_0_43: const #1s : i
        let s_0_43: i128 = 1;
        // C s_0_44: const #10384u : u32
        let s_0_44: u32 = 10384;
        // D s_0_45: read-reg s_0_44:i
        let s_0_45: i128 = {
            let value = state.read_register::<i128>(s_0_44 as isize);
            tracer.read_register(s_0_44 as isize, value);
            value
        };
        // D s_0_46: sub s_0_45 s_0_43
        let s_0_46: i128 = ((s_0_45) - (s_0_43));
        // D s_0_47: cast reint s_0_46 -> i64
        let s_0_47: i64 = (s_0_46 as i64);
        // D s_0_48: write-var gs#26033 <= s_0_47
        fn_state.gs_26033 = s_0_47;
        // D s_0_49: write-var i <= s_0_42
        fn_state.i = s_0_42;
        // N s_0_50: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var i:i64
        let s_1_0: i64 = fn_state.i;
        // D s_1_1: read-var gs#26033:i64
        let s_1_1: i64 = fn_state.gs_26033;
        // D s_1_2: cmp-gt s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) > (s_1_1));
        // N s_1_3: branch s_1_2 b33 b2
        if s_1_2 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #13704u : u32
        let s_2_0: u32 = 13704;
        // D s_2_1: read-reg s_2_0:struct
        let s_2_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call _get_PMBSR_EL1_Type_S(s_2_1)
        let s_2_2: bool = u_get_PMBSR_EL1_Type_S(state, tracer, s_2_1);
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // C s_2_4: const #0u : u8
        let s_2_4: bool = false;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 1u16);
        // D s_2_6: cmp-eq s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) == (s_2_5));
        // N s_2_7: branch s_2_6 b5 b3
        if s_2_6 {
            return block_5(state, tracer, fn_state);
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
        // D s_4_0: read-var i:i64
        let s_4_0: i64 = fn_state.i;
        // C s_4_1: const #1s : i64
        let s_4_1: i64 = 1;
        // D s_4_2: add s_4_0 s_4_1
        let s_4_2: i64 = (s_4_0 + s_4_1);
        // D s_4_3: write-var i <= s_4_2
        fn_state.i = s_4_2;
        // N s_4_4: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var i:i64
        let s_5_0: i64 = fn_state.i;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: call __id(s_5_1)
        let s_5_2: i128 = u__id(state, tracer, s_5_1);
        // C s_5_3: const #0s : i
        let s_5_3: i128 = 0;
        // D s_5_4: cmp-le s_5_3 s_5_2
        let s_5_4: bool = ((s_5_3) <= (s_5_2));
        // N s_5_5: branch s_5_4 b32 b6
        if s_5_4 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#26039 <= s_6_0
        fn_state.gs_26039 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#26039:u8
        let s_7_0: bool = fn_state.gs_26039;
        // N s_7_1: assert s_7_0
        let s_7_1: () = assert!(s_7_0);
        // C s_7_2: const #90512u : u32
        let s_7_2: u32 = 90512;
        // D s_7_3: read-reg s_7_2:u64
        let s_7_3: u64 = {
            let value = state.read_register::<u64>(s_7_2 as isize);
            tracer.read_register(s_7_2 as isize, value);
            value
        };
        // C s_7_4: const #0s : i
        let s_7_4: i128 = 0;
        // D s_7_5: cast zx s_7_3 -> bv
        let s_7_5: Bits = Bits::new(s_7_3 as u128, 64u16);
        // C s_7_6: const #1s : i64
        let s_7_6: i64 = 1;
        // C s_7_7: cast zx s_7_6 -> i
        let s_7_7: i128 = (i128::try_from(s_7_6).unwrap());
        // C s_7_8: const #63s : i
        let s_7_8: i128 = 63;
        // C s_7_9: add s_7_8 s_7_7
        let s_7_9: i128 = (s_7_8 + s_7_7);
        // D s_7_10: bit-extract s_7_5 s_7_4 s_7_9
        let s_7_10: Bits = (Bits::new(
            ((s_7_5) >> (s_7_4)).value(),
            u16::try_from(s_7_9).unwrap(),
        ));
        // D s_7_11: cast reint s_7_10 -> u64
        let s_7_11: u64 = (s_7_10.value() as u64);
        // C s_7_12: const #10664u : u32
        let s_7_12: u32 = 10664;
        // D s_7_13: read-reg s_7_12:[u8; 64]
        let s_7_13: [u8; 64usize] = {
            let value = state.read_register::<[u8; 64usize]>(s_7_12 as isize);
            tracer.read_register(s_7_12 as isize, value);
            value
        };
        // D s_7_14: read-var i:i64
        let s_7_14: i64 = fn_state.i;
        // D s_7_15: cast zx s_7_14 -> i
        let s_7_15: i128 = (i128::try_from(s_7_14).unwrap());
        // D s_7_16: read-element s_7_13[s_7_15]
        let s_7_16: u8 = s_7_13[(s_7_15) as usize];
        // D s_7_17: read-var accdesc:struct
        let s_7_17: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_7_18: read-var aligned:u8
        let s_7_18: bool = fn_state.aligned;
        // D s_7_19: call DebugMemWrite(s_7_11, s_7_17, s_7_18, s_7_16)
        let s_7_19: ProductType7b38a52e3b2f4e94 = DebugMemWrite(
            state,
            tracer,
            s_7_11,
            s_7_17,
            s_7_18,
            s_7_16,
        );
        // D s_7_20: write-var ga#20363 <= s_7_19
        fn_state.ga_20363 = s_7_19;
        // N s_7_21: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ga#20363.0:struct
        let s_8_0: ProductTypef8c3639b88223255 = fn_state.ga_20363._0;
        // D s_8_1: read-var ga#20363.1:struct
        let s_8_1: ProductTypece7c66ccb2cab13e = fn_state.ga_20363._1;
        // D s_8_2: write-var memstatus <= s_8_0
        fn_state.memstatus = s_8_0;
        // D s_8_3: write-var addrdesc <= s_8_1
        fn_state.addrdesc = s_8_1;
        // D s_8_4: read-var addrdesc.0:struct
        let s_8_4: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_8_5: write-var fault <= s_8_4
        fn_state.fault = s_8_4;
        // D s_8_6: read-var fault.16:struct
        let s_8_6: u32 = fn_state.fault._16;
        // C s_8_7: const #9u : u32
        let s_8_7: u32 = 9;
        // D s_8_8: cmp-eq s_8_6 s_8_7
        let s_8_8: bool = ((s_8_6) == (s_8_7));
        // N s_8_9: branch s_8_8 b31 b9
        if s_8_8 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var fault.16:struct
        let s_9_0: u32 = fn_state.fault._16;
        // C s_9_1: const #11u : u32
        let s_9_1: u32 = 11;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // D s_9_3: write-var gs#26048 <= s_9_2
        fn_state.gs_26048 = s_9_2;
        // N s_9_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#26048:u8
        let s_10_0: bool = fn_state.gs_26048;
        // D s_10_1: write-var ttw_fault <= s_10_0
        fn_state.ttw_fault = s_10_0;
        // D s_10_2: read-var fault.16:struct
        let s_10_2: u32 = fn_state.fault._16;
        // D s_10_3: call IsFault__1(s_10_2)
        let s_10_3: bool = IsFault__1(state, tracer, s_10_2);
        // N s_10_4: branch s_10_3 b27 b11
        if s_10_3 {
            return block_27(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#26050 <= s_11_0
        fn_state.gs_26050 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#26050:u8
        let s_12_0: bool = fn_state.gs_26050;
        // N s_12_1: branch s_12_0 b26 b13
        if s_12_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var memstatus:struct
        let s_13_0: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_13_1: call IsFault__2(s_13_0)
        let s_13_1: bool = IsFault__2(state, tracer, s_13_0);
        // N s_13_2: branch s_13_1 b25 b14
        if s_13_1 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var ttw_fault:u8
        let s_14_0: bool = fn_state.ttw_fault;
        // N s_14_1: branch s_14_0 b24 b15
        if s_14_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#26054 <= s_15_0
        fn_state.gs_26054 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#26054:u8
        let s_16_0: bool = fn_state.gs_26054;
        // D s_16_1: write-var gs#26055 <= s_16_0
        fn_state.gs_26055 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#26055:u8
        let s_17_0: bool = fn_state.gs_26055;
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
        // C s_19_0: const #13704u : u32
        let s_19_0: u32 = 13704;
        // D s_19_1: read-reg s_19_0:struct
        let s_19_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call _get_PMBSR_EL1_Type_S(s_19_1)
        let s_19_2: bool = u_get_PMBSR_EL1_Type_S(state, tracer, s_19_1);
        // D s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // D s_19_4: call IsZero(s_19_3)
        let s_19_4: bool = IsZero(state, tracer, s_19_3);
        // N s_19_5: branch s_19_4 b22 b20
        if s_19_4 {
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
        // N s_21_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #90512u : u32
        let s_22_0: u32 = 90512;
        // D s_22_1: read-reg s_22_0:u64
        let s_22_1: u64 = {
            let value = state.read_register::<u64>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // C s_22_2: const #1s : i
        let s_22_2: i128 = 1;
        // D s_22_3: cast zx s_22_1 -> bv
        let s_22_3: Bits = Bits::new(s_22_1 as u128, 64u16);
        // C s_22_4: cast cvt s_22_2 -> bv
        let s_22_4: Bits = Bits::new(s_22_2 as u128, 128);
        // D s_22_5: add s_22_3 s_22_4
        let s_22_5: Bits = (s_22_3 + s_22_4);
        // D s_22_6: cast reint s_22_5 -> u64
        let s_22_6: u64 = (s_22_5.value() as u64);
        // D s_22_7: call Mk_PMBPTR_EL1_Type(s_22_6)
        let s_22_7: ProductType5c790c8ef59cc8b2 = Mk_PMBPTR_EL1_Type(
            state,
            tracer,
            s_22_6,
        );
        // C s_22_8: const #90512u : u32
        let s_22_8: u32 = 90512;
        // N s_22_9: write-reg s_22_8 <= s_22_7
        let s_22_9: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_22_8 as isize, s_22_7);
            tracer.write_register(s_22_8 as isize, s_22_7);
        };
        // N s_22_10: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var memstatus:struct
        let s_23_0: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_23_1: read-var addrdesc:struct
        let s_23_1: ProductTypece7c66ccb2cab13e = fn_state.addrdesc;
        // D s_23_2: read-var start_vaddr:u64
        let s_23_2: u64 = fn_state.start_vaddr;
        // D s_23_3: call DebugWriteExternalAbort(s_23_0, s_23_1, s_23_2)
        let s_23_3: () = DebugWriteExternalAbort(state, tracer, s_23_0, s_23_1, s_23_2);
        // N s_23_4: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var ttw_fault_as_external_abort:u8
        let s_24_0: bool = fn_state.ttw_fault_as_external_abort;
        // D s_24_1: write-var gs#26054 <= s_24_0
        fn_state.gs_26054 = s_24_0;
        // N s_24_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#26055 <= s_25_0
        fn_state.gs_26055 = s_25_0;
        // N s_25_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #90512u : u32
        let s_26_0: u32 = 90512;
        // D s_26_1: read-reg s_26_0:u64
        let s_26_1: u64 = {
            let value = state.read_register::<u64>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // C s_26_2: const #0s : i
        let s_26_2: i128 = 0;
        // D s_26_3: cast zx s_26_1 -> bv
        let s_26_3: Bits = Bits::new(s_26_1 as u128, 64u16);
        // C s_26_4: const #1s : i64
        let s_26_4: i64 = 1;
        // C s_26_5: cast zx s_26_4 -> i
        let s_26_5: i128 = (i128::try_from(s_26_4).unwrap());
        // C s_26_6: const #63s : i
        let s_26_6: i128 = 63;
        // C s_26_7: add s_26_6 s_26_5
        let s_26_7: i128 = (s_26_6 + s_26_5);
        // D s_26_8: bit-extract s_26_3 s_26_2 s_26_7
        let s_26_8: Bits = (Bits::new(
            ((s_26_3) >> (s_26_2)).value(),
            u16::try_from(s_26_7).unwrap(),
        ));
        // D s_26_9: cast reint s_26_8 -> u64
        let s_26_9: u64 = (s_26_8.value() as u64);
        // D s_26_10: read-var fault:struct
        let s_26_10: ProductType1d757adad216cdef = fn_state.fault;
        // D s_26_11: call DebugWriteFault(s_26_9, s_26_10)
        let s_26_11: () = DebugWriteFault(state, tracer, s_26_9, s_26_10);
        // N s_26_12: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var ttw_fault:u8
        let s_27_0: bool = fn_state.ttw_fault;
        // N s_27_1: branch s_27_0 b30 b28
        if s_27_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#26049 <= s_28_0
        fn_state.gs_26049 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#26049:u8
        let s_29_0: bool = fn_state.gs_26049;
        // D s_29_1: not s_29_0
        let s_29_1: bool = !s_29_0;
        // D s_29_2: write-var gs#26050 <= s_29_1
        fn_state.gs_26050 = s_29_1;
        // N s_29_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var ttw_fault_as_external_abort:u8
        let s_30_0: bool = fn_state.ttw_fault_as_external_abort;
        // D s_30_1: write-var gs#26049 <= s_30_0
        fn_state.gs_26049 = s_30_0;
        // N s_30_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: write-var gs#26048 <= s_31_0
        fn_state.gs_26048 = s_31_0;
        // N s_31_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var i:i64
        let s_32_0: i64 = fn_state.i;
        // D s_32_1: cast zx s_32_0 -> i
        let s_32_1: i128 = (i128::try_from(s_32_0).unwrap());
        // D s_32_2: call __id(s_32_1)
        let s_32_2: i128 = u__id(state, tracer, s_32_1);
        // C s_32_3: const #64s : i
        let s_32_3: i128 = 64;
        // D s_32_4: cmp-lt s_32_2 s_32_3
        let s_32_4: bool = ((s_32_2) < (s_32_3));
        // D s_32_5: write-var gs#26039 <= s_32_4
        fn_state.gs_26039 = s_32_4;
        // N s_32_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: return
        return;
    }
}
